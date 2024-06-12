
It looks good to me. Now let us look at the next part of the program:

### Part 2

```c
/* Types of mail header continuation lines:
 */
typedef enum {
	hdr_ParagraphStart	= -1,
	hdr_NonHeader		= 0,
	hdr_Header		= 1,
	hdr_Continuation	= 2
} HdrType;
/* Process a stream. This is where the real work happens,
 * except that centering is handled separately.
 */
static void
process_stream(FILE *stream, const char *name)
{
	const char *wordp, *cp;
	wchar_t wc;
	size_t np;
	size_t last_indent = SILLY;	/* how many spaces in last indent? */
	size_t para_line_number = 0;	/* how many lines already read in this para? */
	size_t first_indent = SILLY;	/* indentation of line 0 of paragraph */
	int wcl;			/* number of bytes in wide character */
	int wcw;			/* display width of wide character */
	int word_length;		/* number of bytes in word */
	int word_width;			/* display width of word */
	int space_width;		/* display width of space after word */
	int line_width;			/* display width of line */
	HdrType prev_header_type = hdr_ParagraphStart;
	HdrType header_type;

	/* ^-- header_type of previous line; -1 at para start */
	const char *line;

	if (centerP) {
		center_stream(stream, name);
		return;
	}

	while ((line = get_line(stream)) != NULL) {
		np = indent_length(line);
		header_type = hdr_NonHeader;
		if (grok_mail_headers && prev_header_type != hdr_NonHeader) {
			if (np == 0 && might_be_header(line))
				header_type = hdr_Header;
			else if (np > 0 && prev_header_type>hdr_NonHeader)
				header_type = hdr_Continuation;
		}

		/* We need a new paragraph if and only if:
		 *   this line is blank,
		 *   OR it's a troff request,
		 *   OR it's a mail header,
		 *   OR it's not a mail header AND the last line was one,
		 *   OR the indentation has changed
		 *      AND the line isn't a mail header continuation line
		 *      AND this isn't the second line of an indented paragraph.
		 */
		if (*line == '\0' || (*line == '.' && !format_troff) ||
		    header_type == hdr_Header ||
		    (header_type == hdr_NonHeader && prev_header_type > hdr_NonHeader) ||
		    (np != last_indent && header_type != hdr_Continuation &&
		    (!allow_indented_paragraphs || para_line_number != 1)) ) {
			new_paragraph(np);
			para_line_number = 0;
			first_indent = np;
			last_indent = np;

			/* nroff compatibility */
			if (*line == '.' && !format_troff) {
				puts(line);
				continue;
			}
			if (header_type == hdr_Header)
				last_indent = 2;	/* for cont. lines */
			if (*line == '\0') {
				putchar('\n');
				prev_header_type = hdr_ParagraphStart;
				continue;
			} else {
				/* If this is an indented paragraph other than a mail header
				 * continuation, set |last_indent|.
				 */
				if (np != last_indent && header_type != hdr_Continuation)
					last_indent = np;
			}
			prev_header_type = header_type;
		}

		line_width = np;
		for (wordp = line; *wordp != '\0'; wordp = cp) {
			word_length = 0;
			word_width = space_width = 0;
			for (cp = wordp; *cp != '\0'; cp += wcl) {
				wcl = mbtowc(&wc, cp, MB_CUR_MAX);
				if (wcl == -1) {
					(void)mbtowc(NULL, NULL, MB_CUR_MAX);
					wc = L'?';
					wcl = 1;
					wcw = 1;
				} else if (wc == L'\t')
					wcw = (line_width / tab_width + 1) *
					    tab_width - line_width;
				else if ((wcw = wcwidth(wc)) == -1)
					wcw = 1;
				if (iswblank(wc) && wc != 0xa0) {
					/* Skip whitespace at start of line. */
					if (word_length == 0) {
						wordp += wcl;
						continue;
					}
					/* Count whitespace after word. */
					space_width += wcw;
				} else {
					/* Detect end of word. */
					if (space_width > 0)
						break;
					/* Measure word. */
					word_length += wcl;
					word_width += wcw;
				}
				line_width += wcw;
			}

			/* Send the word to the output machinery. */
			output_word(first_indent, last_indent, wordp,
			    word_length, word_width, space_width);
		}
		++para_line_number;
	}

	new_paragraph(0);
	if (ferror(stream)) {
		warn("%s", name);
		ERRS(n_errors);
	}
}
```

Please translate translate Part 2 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
