use cursive::{Cursive, views::{Dialog, TextView}, event::Key, menu, With};
use std::sync::atomic::{AtomicUsize, Ordering};

pub(crate) struct Blaze {}

pub fn start_tui()  {
    tui()
}

fn tui () {
    let mut siv = cursive::default();

    // We can quit by pressing `q`
    siv.add_global_callback('q', Cursive::quit);

    // Add a simple view
    // siv.add_layer(TextView::new(
    //     "Hello World!\n\
    //      Press q to quit the application.",
    // ));
     let counter = AtomicUsize::new(1);


    siv.menubar()
        // We add a new "File" tree
        .add_subtree(
            "File",
            menu::Tree::new()
                // Trees are made of leaves, with are directly actionable...
                .leaf("New", move |s| {
                    // Here we use the counter to add an entry
                    // in the list of "Recent" items.
                    let i = counter.fetch_add(1, Ordering::Relaxed);
                    let filename = format!("New {i}");
                    s.menubar()
                        .find_subtree("File")
                        .unwrap()
                        .find_subtree("Recent")
                        .unwrap()
                        .insert_leaf(0, filename, |_| ());

                    s.add_layer(Dialog::info("New file!"));
                })
                // ... and of sub-trees, which open up when selected.
                .subtree(
                    "Recent",
                    // The `.with()` method can help when running loops
                    // within builder patterns.
                    menu::Tree::new().with(|tree| {
                        for i in 1..100 {
                            // We don't actually do anything here,
                            // but you could!
                            tree.add_item(menu::Item::leaf(format!("Item {i}"), |_| ()).with(|s| {
                                if i % 5 == 0 { s.disable(); }
                            }))
                        }
                    }),
                )
                // Delimiter are simple lines between items,
                // and cannot be selected.
                .delimiter()
                .with(|tree| {
                    for i in 1..10 {
                        tree.add_leaf(format!("Option {i}"), |_| ());
                    }
                }),
        )
        .add_subtree(
            "Help",
            menu::Tree::new()
                .subtree(
                    "Help",
                    menu::Tree::new()
                        .leaf("General", |s| {
                            s.add_layer(Dialog::info("Help message!"))
                        })
                        .leaf("Online", |s| {
                            let text = "Google it yourself!\n\
                                        Kids, these days...";
                            s.add_layer(Dialog::info(text))
                        }),
                )
                .leaf("About", |s| {
                    s.add_layer(Dialog::info("Cursive v0.0.0"))
                }),
        )
        .add_delimiter()
        .add_leaf("Quit", |s| s.quit());

    // When `autohide` is on (default), the menu only appears when active.
    // Turning it off will leave the menu always visible.
    // Try uncommenting this line!

    // siv.set_autohide_menu(false);

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    // Run the event loop
    siv.run();
}
