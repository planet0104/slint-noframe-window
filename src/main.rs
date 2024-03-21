use anyhow::Result;
use slint::LogicalPosition;

slint::include_modules!();

fn main() -> Result<()> {
    let main = Main::new()?;

    let handel = main.as_weak();
    main.on_close_window(move ||{
        handel.upgrade().unwrap().hide().unwrap();
    });

    let handel = main.as_weak();
    main.on_minimized_window(move |enable|{
        handel.upgrade().unwrap().window().set_minimized(enable);
    });

    let handel = main.as_weak();
    main.on_maximized_window(move |enable|{
        handel.upgrade().unwrap().window().set_maximized(enable);
    });

    let handel = main.as_weak();
    main.on_move_window(move |offset_x, offset_y|{
        let main = handel.upgrade().unwrap();
        let logical_pos = main.window().position().to_logical(main.window().scale_factor());
        main.window().set_position(LogicalPosition::new(logical_pos.x + offset_x, logical_pos.y + offset_y));
    });

    main.run()?;
    Ok(())
}