mv ~/.local/share/todo/todo.db ~/.local/share/todo/todo.db.bak
cargo test -- --test-threads=1
RES=$?
rm ~/.local/share/todo/todo.db
mv ~/.local/share/todo/todo.db.bak ~/.local/share/todo/todo.db
exit $RES
