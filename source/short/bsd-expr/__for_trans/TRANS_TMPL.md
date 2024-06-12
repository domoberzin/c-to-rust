``````
The task is to translate the following C program to **safe** Rust that has the same external behavior. 
There are multiple functions in the C program so let us do it piece by piece.
Let us first take a look of the complete program:

### Full Program

```c
{TPL:FULL_PROGRAM_CONTENT}
```

---------

We are splitting the above program into multiple parts. You are expected to output the complete translation of each part I gave you and I will verify it before giving you the task for the next part. I know Rust and the Cargo build system, thus you don't need to explain how to build the program. I will concatenate your outputs to get the final program and build it myself.

Let us first look at Part {TPL:PART_IDX} (a fraction of the full program above):

### Part {TPL:PART_IDX}

```c
{TPL:PART_CONTENT}
```

Please first translate translate Part {TPL:PART_IDX} of the C program (provided above) to Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
``````

``````
It looks good to me. Now let us look at the next part of the program:

### Part {TPL:PART_IDX}

```c
{TPL:PART_CONTENT}
```

Please translate translate Part {TPL:PART_IDX} of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
``````

## Full Program
```c
/*	$OpenBSD: expr.c,v 1.27 2018/03/31 14:50:56 tobias Exp $	*/
/*	$NetBSD: expr.c,v 1.3.6.1 1996/06/04 20:41:47 cgd Exp $	*/

/*
 * Written by J.T. Conklin <jtc@netbsd.org>.
 * Public domain.
 */
#define _GNU_SOURCE
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>
#include <ctype.h>
#include <unistd.h>
#include <regex.h>
#include <err.h>
#include <errno.h>

#define	INVALID		1
#define	TOOSMALL	2
#define	TOOLARGE	3

long long
strtonum(const char *numstr, long long minval, long long maxval,
    const char **errstrp)
{
	long long ll = 0;
	int error = 0;
	char *ep;
	struct errval {
		const char *errstr;
		int err;
	} ev[4] = {
		{ NULL,		0 },
		{ "invalid",	EINVAL },
		{ "too small",	ERANGE },
		{ "too large",	ERANGE },
	};

	ev[0].err = errno;
	errno = 0;
	if (minval > maxval) {
		error = INVALID;
	} else {
		ll = strtoll(numstr, &ep, 10);
		if (numstr == ep || *ep != '\0')
			error = INVALID;
		else if ((ll == LLONG_MIN && errno == ERANGE) || ll < minval)
			error = TOOSMALL;
		else if ((ll == LLONG_MAX && errno == ERANGE) || ll > maxval)
			error = TOOLARGE;
	}
	if (errstrp != NULL)
		*errstrp = ev[error].errstr;
	errno = ev[error].err;
	if (error)
		ll = 0;

	return (ll);
}
///// PART_SPLIT /////
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
///// PART_SPLIT /////

/* coerce to vp to an integer */
int
to_integer(struct val *vp, const char **errstr)
{
	int64_t		r;

	if (errstr != NULL)
		*errstr = NULL;

	if (vp->type == integer)
		return 1;

	if (is_integer(vp, &r, errstr)) {
		free(vp->u.s);
		vp->u.i = r;
		vp->type = integer;
		return 1;
	}

	return 0;
}


/* coerce to vp to an string */
void
to_string(struct val *vp)
{
	char	       *tmp;

	if (vp->type == string)
		return;

	if (asprintf(&tmp, "%lld", vp->u.i) == -1)
		err(3, NULL);

	vp->type = string;
	vp->u.s = tmp;
}

int
is_zero_or_null(struct val *vp)
{
	if (vp->type == integer)
		return vp->u.i == 0;
	else
		return *vp->u.s == 0 || (to_integer(vp, NULL) && vp->u.i == 0);
}

void
nexttoken(int pat)
{
	char	       *p;

	if ((p = *av) == NULL) {
		token = EOI;
		return;
	}
	av++;

	
	if (pat == 0 && p[0] != '\0') {
		if (p[1] == '\0') {
			const char     *x = "|&=<>+-*/%:()";
			char	       *i;	/* index */

			if ((i = strchr(x, *p)) != NULL) {
				token = i - x;
				return;
			}
		} else if (p[1] == '=' && p[2] == '\0') {
			switch (*p) {
			case '<':
				token = LE;
				return;
			case '>':
				token = GE;
				return;
			case '!':
				token = NE;
				return;
			}
		}
	}
	tokval = make_str(p);
	token = OPERAND;
	return;
}

void
error(void)
{
	errx(2, "syntax error");
}
///// PART_SPLIT /////
struct val *
eval6(void)
{
	struct val     *v;

	if (token == OPERAND) {
		nexttoken(0);
		return tokval;
	} else if (token == RP) {
		nexttoken(0);
		v = eval0();
		if (token != LP)
			error();
		nexttoken(0);
		return v;
	} else
		error();
}

/* Parse and evaluate match (regex) expressions */
struct val *
eval5(void)
{
	regex_t		rp;
	regmatch_t	rm[2];
	char		errbuf[256];
	int		eval;
	struct val     *l, *r;
	struct val     *v;

	l = eval6();
	while (token == MATCH) {
		nexttoken(1);
		r = eval6();

		/* coerce to both arguments to strings */
		to_string(l);
		to_string(r);

		/* compile regular expression */
		if ((eval = regcomp(&rp, r->u.s, 0)) != 0) {
			regerror(eval, &rp, errbuf, sizeof(errbuf));
			errx(2, "%s", errbuf);
		}

		/* compare string against pattern --  remember that patterns
		   are anchored to the beginning of the line */
		if (regexec(&rp, l->u.s, 2, rm, 0) == 0 && rm[0].rm_so == 0) {
			if (rm[1].rm_so >= 0) {
				*(l->u.s + rm[1].rm_eo) = '\0';
				v = make_str(l->u.s + rm[1].rm_so);

			} else {
				v = make_int(rm[0].rm_eo - rm[0].rm_so);
			}
		} else {
			if (rp.re_nsub == 0) {
				v = make_int(0);
			} else {
				v = make_str("");
			}
		}

		/* free arguments and pattern buffer */
		free_value(l);
		free_value(r);
		regfree(&rp);

		l = v;
	}

	return l;
}

/* Parse and evaluate multiplication and division expressions */
struct val *
eval4(void)
{
	const char	*errstr;
	struct val	*l, *r;
	enum token	 op;
	volatile int64_t res;

	l = eval5();
	while ((op = token) == MUL || op == DIV || op == MOD) {
		nexttoken(0);
		r = eval5();

		if (!to_integer(l, &errstr))
			errx(2, "number \"%s\" is %s", l->u.s, errstr);
		if (!to_integer(r, &errstr))
			errx(2, "number \"%s\" is %s", r->u.s, errstr);

		if (op == MUL) {
			res = l->u.i * r->u.i;
			if (r->u.i != 0 && l->u.i != res / r->u.i)
				errx(3, "overflow");
			l->u.i = res;
		} else {
			if (r->u.i == 0) {
				errx(2, "division by zero");
			}
			if (op == DIV) {
				if (l->u.i != INT64_MIN || r->u.i != -1)
					l->u.i /= r->u.i;
				else
					errx(3, "overflow");
			} else {
				if (l->u.i != INT64_MIN || r->u.i != -1)
					l->u.i %= r->u.i;
				else
					l->u.i = 0;
			}
		}

		free_value(r);
	}

	return l;
}
///// PART_SPLIT /////
/* Parse and evaluate addition and subtraction expressions */
struct val *
eval3(void)
{
	const char	*errstr;
	struct val	*l, *r;
	enum token	 op;
	volatile int64_t res;

	l = eval4();
	while ((op = token) == ADD || op == SUB) {
		nexttoken(0);
		r = eval4();

		if (!to_integer(l, &errstr))
			errx(2, "number \"%s\" is %s", l->u.s, errstr);
		if (!to_integer(r, &errstr))
			errx(2, "number \"%s\" is %s", r->u.s, errstr);

		if (op == ADD) {
			res = l->u.i + r->u.i;
			if ((l->u.i > 0 && r->u.i > 0 && res <= 0) ||
			    (l->u.i < 0 && r->u.i < 0 && res >= 0))
				errx(3, "overflow");
			l->u.i = res;
		} else {
			res = l->u.i - r->u.i;
			if ((l->u.i >= 0 && r->u.i < 0 && res <= 0) ||
			    (l->u.i < 0 && r->u.i > 0 && res >= 0))
				errx(3, "overflow");
			l->u.i = res;
		}

		free_value(r);
	}

	return l;
}

/* Parse and evaluate comparison expressions */
struct val *
eval2(void)
{
	struct val     *l, *r;
	enum token	op;
	int64_t		v = 0, li, ri;

	l = eval3();
	while ((op = token) == EQ || op == NE || op == LT || op == GT ||
	    op == LE || op == GE) {
		nexttoken(0);
		r = eval3();

		if (is_integer(l, &li, NULL) && is_integer(r, &ri, NULL)) {
			switch (op) {
			case GT:
				v = (li >  ri);
				break;
			case GE:
				v = (li >= ri);
				break;
			case LT:
				v = (li <  ri);
				break;
			case LE:
				v = (li <= ri);
				break;
			case EQ:
				v = (li == ri);
				break;
			case NE:
				v = (li != ri);
				break;
			default:
				break;
			}
		} else {
			to_string(l);
			to_string(r);

			switch (op) {
			case GT:
				v = (strcoll(l->u.s, r->u.s) > 0);
				break;
			case GE:
				v = (strcoll(l->u.s, r->u.s) >= 0);
				break;
			case LT:
				v = (strcoll(l->u.s, r->u.s) < 0);
				break;
			case LE:
				v = (strcoll(l->u.s, r->u.s) <= 0);
				break;
			case EQ:
				v = (strcoll(l->u.s, r->u.s) == 0);
				break;
			case NE:
				v = (strcoll(l->u.s, r->u.s) != 0);
				break;
			default:
				break;
			}
		}

		free_value(l);
		free_value(r);
		l = make_int(v);
	}

	return l;
}
///// PART_SPLIT /////
/* Parse and evaluate & expressions */
struct val *
eval1(void)
{
	struct val     *l, *r;

	l = eval2();
	while (token == AND) {
		nexttoken(0);
		r = eval2();

		if (is_zero_or_null(l) || is_zero_or_null(r)) {
			free_value(l);
			free_value(r);
			l = make_int(0);
		} else {
			free_value(r);
		}
	}

	return l;
}

/* Parse and evaluate | expressions */
struct val *
eval0(void)
{
	struct val     *l, *r;

	l = eval1();
	while (token == OR) {
		nexttoken(0);
		r = eval1();

		if (is_zero_or_null(l)) {
			free_value(l);
			l = r;
		} else {
			free_value(r);
		}
	}

	return l;
}


int
main(int argc, char *argv[])
{
	struct val     *vp;

	if (argc > 1 && !strcmp(argv[1], "--"))
		argv++;

	av = argv + 1;

	nexttoken(0);
	vp = eval0();

	if (token != EOI)
		error();

	if (vp->type == integer)
		printf("%lld\n", vp->u.i);
	else
		printf("%s\n", vp->u.s);

	return is_zero_or_null(vp);
}
```
