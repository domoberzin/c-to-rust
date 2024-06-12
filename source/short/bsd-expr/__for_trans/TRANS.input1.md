
It looks good to me. Now let us look at the next part of the program:

### Part 1

```c
struct val	*make_int(int64_t);
struct val	*make_str(char *);
void		 free_value(struct val *);
int		 is_integer(struct val *, int64_t *, const char **);
int		 to_integer(struct val *, const char **);
void		 to_string(struct val *);
int		 is_zero_or_null(struct val *);
void		 nexttoken(int);
void	 error(void);
struct val	*eval6(void);
struct val	*eval5(void);
struct val	*eval4(void);
struct val	*eval3(void);
struct val	*eval2(void);
struct val	*eval1(void);
struct val	*eval0(void);

enum token {
	OR, AND, EQ, LT, GT, ADD, SUB, MUL, DIV, MOD, MATCH, RP, LP,
	NE, LE, GE, OPERAND, EOI
};

struct val {
	enum {
		integer,
		string
	} type;

	union {
		char	       *s;
		int64_t		i;
	} u;
};

enum token	token;
struct val     *tokval;
char	      **av;

struct val *
make_int(int64_t i)
{
	struct val     *vp;

	vp = malloc(sizeof(*vp));
	if (vp == NULL) {
		err(3, NULL);
	}
	vp->type = integer;
	vp->u.i = i;
	return vp;
}


struct val *
make_str(char *s)
{
	struct val     *vp;

	vp = malloc(sizeof(*vp));
	if (vp == NULL || ((vp->u.s = strdup(s)) == NULL)) {
		err(3, NULL);
	}
	vp->type = string;
	return vp;
}


void
free_value(struct val *vp)
{
	if (vp->type == string)
		free(vp->u.s);
	free(vp);
}


/* determine if vp is an integer; if so, return it's value in *r */
int
is_integer(struct val *vp, int64_t *r, const char **errstr)
{
	const char *errstrp;

	if (errstr == NULL)
		errstr = &errstrp;
	*errstr = NULL;

	if (vp->type == integer) {
		*r = vp->u.i;
		return 1;
	}

	/*
	 * POSIX.2 defines an "integer" as an optional unary minus
	 * followed by digits. Other representations are unspecified,
	 * which means that strtonum(3) is a viable option here.
	 */
	*r = strtonum(vp->u.s, INT64_MIN, INT64_MAX, errstr);
	return *errstr == NULL;
}
```

Please translate translate Part 1 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
