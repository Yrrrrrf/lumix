use windows::Win32::Graphics::Gdi;
use windows::Win32::Foundation::{LPARAM, BOOL, RECT};
use windows::Win32::Devices::Display::{
    SetMonitorBrightness, GetPhysicalMonitorsFromHMONITOR,
    PHYSICAL_MONITOR, GetMonitorBrightness,
};

// ANSI color codes and formatting
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";
const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const RED: &str = "\x1b[31m";

struct MonitorControl {
    monitors: Vec<(isize, PHYSICAL_MONITOR)>,
}

impl MonitorControl {
    fn new() -> Self { MonitorControl { monitors: Vec::new() } }

    fn add_monitor(&mut self, handle: isize, physical_monitor: PHYSICAL_MONITOR) {
        self.monitors.push((handle, physical_monitor));
    }

    unsafe fn set_brightness(&self, handle: Option<isize>, brightness: u32) -> Result<(), String> {
        assert!(brightness <= 100, "Brightness must be between 0 and 100");

        match handle {
            Some(h) => {
                if let Some((_, monitor)) = self.monitors.iter().find(|(handle, _)| *handle == h) {
                    SetMonitorBrightness(monitor.hPhysicalMonitor, brightness);
                    println!("{GREEN}✓{RESET} Set brightness to {BOLD}{brightness}%{RESET} for monitor M_{h}");
                    Ok(())
                } else {Err(format!("{}✗{} Monitor M_{} not found", RED, RESET, h))}
            }
            None => {
                // Set brightness for all monitors
                for (handle, monitor) in &self.monitors {
                    SetMonitorBrightness(monitor.hPhysicalMonitor, brightness);
                    println!("{GREEN}✓{RESET} Set brightness to {BOLD}{brightness}%{RESET} for monitor M_{handle}");
                }
                Ok(())
            }
        }
    }

    unsafe fn get_brightness(&self, handle: Option<isize>) {
        let print_monitor = |(handle, monitor): &(isize, PHYSICAL_MONITOR)| {
            let mut min = 0;
            let mut current = 0;
            let mut max = 0;
            GetMonitorBrightness(monitor.hPhysicalMonitor, &mut min, &mut current, &mut max);
            // Create a simple progress bar
            let bar_width = 20;
            let filled = (current as f32 / max as f32 * bar_width as f32) as usize;
            let bar: String = format!("{}{}{}", "█".repeat(filled), DIM, "░".repeat(bar_width - filled));
            println!("{CYAN}Monitor {handle}{RESET}: {BOLD}{current:>3}%{RESET} {DIM}[{min}..{max:>3}]{RESET} {BLUE}{bar}{RESET}");
        };

        match handle {
            Some(h) => {
                if let Some(monitor) = self.monitors.iter().find(|(handle, _)| *handle == h) {
                    print_monitor(monitor);
                } else {println!("{}✗{} Monitor {} not found", RED, RESET, h);}
            }
            None => {
                println!("\n{}Monitor Brightness Status:{}\n", BOLD, RESET);
                for monitor in &self.monitors {print_monitor(monitor);}
                println!(); // Add extra newline for spacing
            }
        }
    }
}

fn print_usage() {
    println!("\n{}Lumix - Monitor Brightness Control{}\n", BOLD, RESET);
    println!("{}Commands:{}", YELLOW, RESET);
    println!("  {}lumix get [monitor_handle]     {}- Get brightness of specific monitor or all monitors", BOLD, RESET);
    println!("  {}lumix set <brightness>         {}- Set brightness for all monitors", BOLD, RESET);
    println!("  {}lumix set <monitor_handle> <brightness> {}- Set brightness for specific monitor", BOLD, RESET);
    println!("\n{}Examples:{}", YELLOW, RESET);
    println!("  {}$ lumix list", DIM);
    println!("  $ lumix get");
    println!("  $ lumix get 12345");
    println!("  $ lumix set 75");
    println!("  $ lumix set 12345 50{}", RESET);
    println!(); // Add extra newline for spacing
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 { print_usage(); return; }

    let mut monitor_control = MonitorControl::new();

    unsafe {
        unsafe extern "system" fn handle_monitor(
            param0: Gdi::HMONITOR,
            _: Gdi::HDC,
            _: *mut RECT,
            monitor_control: LPARAM,
        ) -> BOOL {
            let control = &mut *(monitor_control.0 as *mut MonitorControl);
            let mut physical_monitor = [PHYSICAL_MONITOR::default(); 1];
            _ = GetPhysicalMonitorsFromHMONITOR(param0, &mut physical_monitor);
            let handle = param0.0 as isize;
            control.add_monitor(handle, physical_monitor[0]);
            BOOL(1)
        }

        _ = Gdi::EnumDisplayMonitors(None, None, Some(handle_monitor), LPARAM(&mut monitor_control as *mut _ as isize));

        match args[1].as_str() {
            "get" => monitor_control.get_brightness(args.get(2).and_then(|s| s.parse().ok())),
            "set" => {
                match args.len() {
                    3 => {
                        if let Ok(brightness) = args[2].parse() {
                            if let Err(e) = monitor_control.set_brightness(None, brightness) {
                                println!("{}Error:{} {}", RED, RESET, e);
                            }
                        } else {println!("{}Error:{} Invalid brightness value", RED, RESET);}
                    }
                    4 => {
                        if let (Ok(handle), Ok(brightness)) = (args[2].parse(), args[3].parse()) {
                            if let Err(e) = monitor_control.set_brightness(Some(handle), brightness) {
                                println!("{}Error:{} {}", RED, RESET, e);
                            }
                        } else {println!("{}Error:{} Invalid handle or brightness value", RED, RESET);}
                    }
                    _ => print_usage(),
                }
            }
            _ => print_usage(),
        }
    }
}
