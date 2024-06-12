
It looks good to me. Now let us look at the next part of the program:

### Part 2

```c
void
slurp(INPUT *F)
{
	LINE *lp, *lastlp, tmp;
	ssize_t len;
	u_long cnt;
	char *bp, *fieldp;

	/*
	 * Read all of the lines from an input file that have the same
	 * join field.
	 */

	F->setcnt = 0;
	for (lastlp = NULL; ; ++F->setcnt) {
		/*
		 * If we're out of space to hold line structures, allocate
		 * more.  Initialize the structure so that we know that this
		 * is new space.
		 */
		if (F->setcnt == F->setalloc) {
			LINE *p;
			u_long newsize = F->setalloc + 50;
			cnt = F->setalloc;
			if ((p = reallocarray(F->set, newsize, sizeof(LINE)))
			    == NULL)
				err(1, NULL);
			F->set = p;
			F->setalloc = newsize;
			memset(F->set + cnt, 0, 50 * sizeof(LINE));
			/* re-set lastlp in case it moved */
			if (lastlp != NULL)
				lastlp = &F->set[F->setcnt - 1];
		}
		/*
		 * Get any pushed back line, else get the next line.  Allocate
		 * space as necessary.  If taking the line from the stack swap
		 * the two structures so that we don't lose space allocated to
		 * either structure.  This could be avoided by doing another
		 * level of indirection, but it's probably okay as is.
		 */
		lp = &F->set[F->setcnt];
		if (F->setcnt)
			lastlp = &F->set[F->setcnt - 1];
		if (F->pushbool) {
			tmp = F->set[F->setcnt];
			F->set[F->setcnt] = F->set[F->pushback];
			F->set[F->pushback] = tmp;
			F->pushbool = 0;
			continue;
		}
		if ((len = getline(&(lp->line), &(lp->linealloc), F->fp)) == -1)
			break;

		/* Remove the trailing newline, if any. */
		if (lp->line[len - 1] == '\n')
			lp->line[--len] = '\0';

		/* Split the line into fields, allocate space as necessary. */
		lp->fieldcnt = 0;
		bp = lp->line;
		while ((fieldp = mbssep(&bp, tabchar)) != NULL) {
			if (spans && *fieldp == '\0')
				continue;
			if (lp->fieldcnt == lp->fieldalloc) {
				char **p;
				u_long newsize = lp->fieldalloc + 50;
				if ((p = reallocarray(lp->fields, newsize,
				    sizeof(char *))) == NULL)
					err(1, NULL);
				lp->fields = p;
				lp->fieldalloc = newsize;
			}
			lp->fields[lp->fieldcnt++] = fieldp;
		}

		/* See if the join field value has changed. */
		if (lastlp != NULL && cmp(lp, F->joinf, lastlp, F->joinf)) {
			F->pushbool = 1;
			F->pushback = F->setcnt;
			break;
		}
	}
}
```

Please translate translate Part 2 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
