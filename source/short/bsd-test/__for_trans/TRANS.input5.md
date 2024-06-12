
It looks good to me. Now let us look at the next part of the program:

### Part 5

```c
static enum token
t_lex_type(char *s)
{
	struct t_op const *op = ops;

	if (s == NULL)
		return -1;

	while (op->op_text) {
		if (strcmp(s, op->op_text) == 0)
			return op->op_type;
		op++;
	}
	return -1;
}

static int
filstat(char *nm, enum token mode)
{
	struct stat s;
	mode_t i;

	if (mode == FILSYM) {
#ifdef S_IFLNK
		if (lstat(nm, &s) == 0) {
			i = S_IFLNK;
			goto filetype;
		}
#endif
		return 0;
	}

	if (stat(nm, &s) != 0)
		return 0;

	switch (mode) {
	case FILRD:
		return access(nm, R_OK) == 0;
	case FILWR:
		return access(nm, W_OK) == 0;
	case FILEX:
		return access(nm, X_OK) == 0;
	case FILEXIST:
		return access(nm, F_OK) == 0;
	case FILREG:
		i = S_IFREG;
		goto filetype;
	case FILDIR:
		i = S_IFDIR;
		goto filetype;
	case FILCDEV:
		i = S_IFCHR;
		goto filetype;
	case FILBDEV:
		i = S_IFBLK;
		goto filetype;
	case FILFIFO:
#ifdef S_IFIFO
		i = S_IFIFO;
		goto filetype;
#else
		return 0;
#endif
	case FILSOCK:
#ifdef S_IFSOCK
		i = S_IFSOCK;
		goto filetype;
#else
		return 0;
#endif
	case FILSUID:
		i = S_ISUID;
		goto filebit;
	case FILSGID:
		i = S_ISGID;
		goto filebit;
	case FILSTCK:
		i = S_ISVTX;
		goto filebit;
	case FILGZ:
		return s.st_size > 0L;
	case FILUID:
		return s.st_uid == geteuid();
	case FILGID:
		return s.st_gid == getegid();
	default:
		return 1;
	}

filetype:
	return ((s.st_mode & S_IFMT) == i);

filebit:
	return ((s.st_mode & i) != 0);
}
```

Please translate translate Part 5 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
