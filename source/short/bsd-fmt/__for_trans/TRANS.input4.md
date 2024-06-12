
It looks good to me. Now let us look at the next part of the program:

### Part 4

```c
/* Output a single word.
 * indent0 and indent1 are the indents to use on the first and subsequent
 * lines of a paragraph. They'll often be the same, of course.
 */
static void
output_word(size_t indent0, size_t indent1, const char *word,
    int length, int width, int spaces)
{
	size_t new_x = x + pending_spaces + width;

	/* If either |spaces==0| (at end of line) or |coalesce_spaces_P|
	 * (squashing internal whitespace), then add just one space;
	 * except that if the last character was a sentence-ender we
	 * actually add two spaces.
	 */
	if (coalesce_spaces_P || spaces == 0)
		spaces = strchr(sentence_enders, word[length-1]) ? 2 : 1;

	if (x0 == 0)
		output_indent(output_in_paragraph ? indent1 : indent0);
	else if (new_x > max_length || x >= goal_length ||
	    (new_x > goal_length && new_x-goal_length > goal_length-x)) {
		putchar('\n');
		output_indent(indent1);
		x0 = 0;
		x = indent1;
	} else {
		x0 += pending_spaces;
		x += pending_spaces;
		while (pending_spaces--)
			putchar(' ');
	}
	x0 += width;
	x += width;
	while(length--)
		putchar(*word++);
	pending_spaces = spaces;
	output_in_paragraph = 1;
}

/* Process a stream, but just center its lines rather than trying to
 * format them neatly.
 */
static void
center_stream(FILE *stream, const char *name)
{
	char *line, *cp;
	wchar_t wc;
	size_t l;	/* Display width of the line. */
	int wcw;	/* Display width of one character. */
	int wcl;	/* Length in bytes of one character. */

	while ((line = get_line(stream)) != NULL) {
		l = 0;
		for (cp = line; *cp != '\0'; cp += wcl) {
			if (*cp == '\t')
				*cp = ' ';
			if ((wcl = mbtowc(&wc, cp, MB_CUR_MAX)) == -1) {
				(void)mbtowc(NULL, NULL, MB_CUR_MAX);
				*cp = '?';
				wcl = 1;
				wcw = 1;
			} else if ((wcw = wcwidth(wc)) == -1)
				wcw = 1;
			if (l == 0 && iswspace(wc))
				line += wcl;
			else
				l += wcw;
		}
		while (l < goal_length) {
			putchar(' ');
			l += 2;
		}
		puts(line);
	}

	if (ferror(stream)) {
		warn("%s", name);
		ERRS(n_errors);
	}
}
```

Please translate translate Part 4 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
