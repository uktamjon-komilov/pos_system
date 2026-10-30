//! Headless layout check for the minimum window size: renders the meals screen
//! with the mock data at several window sizes into `docs/screenshots/`, and fails
//! if a button, badge, toggle or label sticks out of its card, form or column.
//!
//! Element queries need Slint's debug info, so run it explicitly:
//! `SLINT_EMIT_DEBUG_INFO=1 cargo test layout_snapshots -- --ignored --nocapture`

use std::collections::VecDeque;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use i_slint_backend_testing::ElementHandle;
use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{EventLoopProxy, Platform, WindowAdapter, WindowEvent};
use slint::{ComponentHandle, LogicalPosition, PhysicalSize, Rgb8Pixel};

use crate::controllers::MealsController;
use crate::repository::MockMealRepository;
use crate::ui::{AppWindow, DishesStore, MealsStore, MealsTab, ModifiersStore};

/// From `Theme.window-min-width/height` (1280x720) up; the window cannot be smaller.
const SIZES: [(u32, u32); 3] = [(1280, 720), (1366, 768), (1920, 1080)];

/// Element kinds that must stay inside their container.
const CHECKED: [&str; 5] = ["Button", "Badge", "PillLabel", "Toggle", "Text"];

type Task = Box<dyn FnOnce() + Send>;
type Tasks = Arc<Mutex<VecDeque<Task>>>;

/// One software-rendered window, plus a task queue that `settle` drains by
/// hand, so the controllers' `spawn_local` replies run without an event loop.
struct HeadlessPlatform {
    window: Rc<MinimalSoftwareWindow>,
    tasks: Tasks,
    start: Instant,
}

impl Platform for HeadlessPlatform {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, slint::PlatformError> {
        Ok(self.window.clone())
    }

    fn duration_since_start(&self) -> Duration {
        self.start.elapsed()
    }

    fn new_event_loop_proxy(&self) -> Option<Box<dyn EventLoopProxy>> {
        Some(Box::new(Proxy(Arc::clone(&self.tasks))))
    }
}

struct Proxy(Tasks);

impl EventLoopProxy for Proxy {
    fn quit_event_loop(&self) -> Result<(), slint::EventLoopError> {
        Ok(())
    }

    fn invoke_from_event_loop(&self, task: Task) -> Result<(), slint::EventLoopError> {
        self.0.lock().unwrap().push_back(task);
        Ok(())
    }
}

/// Runs queued replies and animations long enough for the mock (150 ms per
/// call) to answer a short chain of requests.
fn settle(tasks: &Tasks) {
    let deadline = Instant::now() + Duration::from_millis(700);
    while Instant::now() < deadline {
        let task = tasks.lock().unwrap().pop_front();
        match task {
            Some(task) => task(),
            None => std::thread::sleep(Duration::from_millis(5)),
        }
        slint::platform::update_timers_and_animations();
    }
}

fn render(window: &MinimalSoftwareWindow, (width, height): (u32, u32)) -> Vec<Rgb8Pixel> {
    let mut pixels = vec![Rgb8Pixel::default(); (width * height) as usize];
    window.request_redraw();
    window.draw_if_needed(|renderer| {
        renderer.render(&mut pixels, width as usize);
    });
    pixels
}

fn save_png(window: &MinimalSoftwareWindow, (width, height): (u32, u32), name: &str) {
    let pixels = render(window, (width, height));
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../docs/screenshots");
    std::fs::create_dir_all(&dir).unwrap();
    let file = std::fs::File::create(dir.join(format!("{width}x{height}-{name}.png"))).unwrap();
    let mut encoder = png::Encoder::new(std::io::BufWriter::new(file), width, height);
    encoder.set_color(png::ColorType::Rgb);
    let bytes: Vec<u8> = pixels.iter().flat_map(|p| [p.r, p.g, p.b]).collect();
    encoder
        .write_header()
        .unwrap()
        .write_image_data(&bytes)
        .unwrap();
}

/// Wheel over the middle of the first `container_type` element.
fn scroll(app: &AppWindow, container_type: &str, delta_y: f32) {
    let container = ElementHandle::find_by_element_type_name(app, container_type)
        .next()
        .unwrap();
    let (left, top, right, bottom) = rect(&container);
    let position = LogicalPosition::new((left + right) / 2.0, (top + bottom) / 2.0);
    app.window()
        .dispatch_event(WindowEvent::PointerMoved { position });
    app.window().dispatch_event(WindowEvent::PointerScrolled {
        position,
        delta_x: 0.0,
        delta_y,
    });
}

fn rect(element: &ElementHandle) -> (f32, f32, f32, f32) {
    let position = element.absolute_position();
    let size = element.size();
    (
        position.x,
        position.y,
        position.x + size.width,
        position.y + size.height,
    )
}

/// The largest overflow of one kind of element inside every `container_type`.
/// Columns scroll vertically, so only their horizontal bounds are checked.
struct Finding {
    container: &'static str,
    kind: &'static str,
    worst_px: f32,
    worst_label: String,
}

fn measure(app: &AppWindow, container: &'static str, vertical: bool) -> Vec<Finding> {
    let containers: Vec<_> = ElementHandle::find_by_element_type_name(app, container).collect();
    assert!(
        !containers.is_empty(),
        "no {container} found (debug info missing?)"
    );
    CHECKED
        .iter()
        .map(|&kind| {
            let mut finding = Finding {
                container,
                kind,
                worst_px: 0.0,
                worst_label: String::new(),
            };
            for outer in &containers {
                let (left, top, right, bottom) = rect(outer);
                for inner in outer.query_descendants().match_inherits(kind).find_all() {
                    let (l, t, r, b) = rect(&inner);
                    if r <= l {
                        continue;
                    }
                    let mut px = (left - l).max(r - right);
                    if vertical {
                        px = px.max(top - t).max(b - bottom);
                    }
                    if px > finding.worst_px {
                        finding.worst_px = px;
                        finding.worst_label = inner.accessible_label().unwrap_or_default().into();
                    }
                }
            }
            finding
        })
        .collect()
}

#[test]
#[ignore = "renders screenshots; run with SLINT_EMIT_DEBUG_INFO=1 and --ignored"]
fn layout_snapshots() {
    let window = MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer);
    let tasks = Tasks::default();
    slint::platform::set_platform(Box::new(HeadlessPlatform {
        window: Rc::clone(&window),
        tasks: Arc::clone(&tasks),
        start: Instant::now(),
    }))
    .unwrap();

    let app = AppWindow::new().unwrap();
    MealsController::new(&app, Arc::new(MockMealRepository::with_sample_data()));
    app.show().unwrap();
    // The first render creates the meals page, whose `init` loads the catalog.
    app.window()
        .set_size(PhysicalSize::new(SIZES[0].0, SIZES[0].1));
    render(&window, SIZES[0]);
    settle(&tasks);

    let mut overflows = Vec::new();
    for size in SIZES {
        let mut report = |state: &str, findings: Vec<Finding>| {
            for finding in findings {
                println!(
                    "{}x{} | {state} | {} > {} | {} | {:.1} px {}",
                    size.0,
                    size.1,
                    finding.container,
                    finding.kind,
                    if finding.worst_px > 0.5 { "yes" } else { "no" },
                    finding.worst_px,
                    finding.worst_label,
                );
                if finding.worst_px > 0.5 {
                    overflows.push(format!("{size:?} {state} {}", finding.kind));
                }
            }
        };
        app.window().set_size(PhysicalSize::new(size.0, size.1));

        // Dish grid: "ÐŸÐ»Ð¾Ð² Ð¸ Ð½Ð°Ñ†Ð¸Ð¾Ð½Ð°Ð»ÑŒÐ½Ñ‹Ðµ Ð±Ð»ÑŽÐ´Ð°" lists ÐŸÐ»Ð¾Ð² first; select it.
        let dishes = app.global::<DishesStore>();
        app.global::<MealsStore>()
            .invoke_tab_selected(MealsTab::Dishes);
        settle(&tasks);
        dishes.invoke_category_clicked("cat-hot".into());
        settle(&tasks);
        dishes.invoke_category_clicked("cat-national".into());
        settle(&tasks);
        dishes.invoke_dish_clicked("dish-plov".into());
        settle(&tasks);
        save_png(&window, size, "1-dish-grid");
        report("dish grid", measure(&app, "DishCard", true));

        // Comment form: at the bottom of the details column.
        scroll(&app, "DishMainPage", -5000.0);
        settle(&tasks);
        save_png(&window, size, "2-comment-form");
        report("comment form", measure(&app, "CommentTemplates", true));
        report("comment form", measure(&app, "DishMainPage", false));

        // Limit form: near the top of the details column.
        scroll(&app, "DishMainPage", 5000.0);
        dishes.set_is_editing_limit(true);
        settle(&tasks);
        save_png(&window, size, "3-limit-form");
        report("limit form", measure(&app, "LimitForm", true));
        report("limit form", measure(&app, "DishMainPage", false));

        // Modifiers tab with a modifier selected and its limit form open.
        app.global::<MealsStore>()
            .invoke_tab_selected(MealsTab::Modifiers);
        settle(&tasks);
        let modifiers = app.global::<ModifiersStore>();
        modifiers.invoke_modifier_clicked("mod-cheese".into());
        modifiers.set_is_editing_limit(true);
        settle(&tasks);
        save_png(&window, size, "4-modifiers");
        report("modifiers", measure(&app, "ModifierCard", true));
        report("modifiers", measure(&app, "LimitForm", true));
        report("modifiers", measure(&app, "ModifierDetails", false));
    }
    assert!(overflows.is_empty(), "elements overflow: {overflows:#?}");
}
