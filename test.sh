export DATABASE_URL=sqlite://todo_test.sqlite3
diesel migration run
cargo test --verbose -- --test-threads=1
rm todo_test.sqlite3
