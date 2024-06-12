# compile join.c 
all: join_dbg join_opt join

join: join.c
	gcc -O0 -fprofile-arcs -ftest-coverage -o join join.c

join_dbg: join.c
	gcc -g -O0 -o join_dbg join.c

join_opt: join.c
	gcc -O3 -o join_opt join.c

clean:
	rm -f join_dbg join join_opt *.gcda *.gcno *.gcov