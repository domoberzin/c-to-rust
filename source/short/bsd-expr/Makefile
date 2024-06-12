# compile expr.c 
all: expr_dbg expr_opt expr

expr: expr.c
	gcc -O0 -fprofile-arcs -ftest-coverage -o expr expr.c

expr_dbg: expr.c
	gcc -g -O0 -o expr_dbg expr.c

expr_opt: expr.c
	gcc -O3 -o expr_opt expr.c

clean:
	rm -f expr_dbg expr expr_opt *.gcda *.gcno *.gcov