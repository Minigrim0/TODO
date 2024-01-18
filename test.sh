mv ~/.local/share/todo/todo.db ~/.local/share/todo/todo.db.bak
cargo test -- --test-threads=1
rm ~/.local/share/todo/todo.db
mv ~/.local/share/todo/todo.db.bak ~/.local/share/todo/todo.db
