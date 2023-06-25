export DATABASE_URL=sqlite://todo_test.sqlite3
diesel migration run
cargo test -- --test-threads=1
RES=$?
rm todo_test.sqlite3
echo "Result of the tests $RES"
exit $RES
