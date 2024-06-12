
It looks good to me. Now let us look at the next part of the program:

### Part 1

```c
int
main(int argc, char *argv[])
{
	char *fmt, *start;
	int havefieldwidth, haveprecision;
	int fieldwidth, precision;
	char convch, nextch;
	char *format;

	/* Need to accept/ignore "--" option. */
	if (argc > 1 && strcmp(argv[1], "--") == 0) {
		argc--;
		argv++;
	}

	if (argc < 2)
		usage();

	format = *++argv;
	gargv = ++argv;

#define SKIP1	"#-+ 0"
#define SKIP2	"0123456789"
	do {
		/*
		 * Basic algorithm is to scan the format string for conversion
		 * specifications -- once one is found, find out if the field
		 * width or precision is a '*'; if it is, gather up value. 
		 * Note, format strings are reused as necessary to use up the
		 * provided arguments, arguments of zero/null string are 
		 * provided to use up the format string.
		 */

		/* find next format specification */
		for (fmt = format; *fmt; fmt++) {
			switch (*fmt) {
			case '%':
				start = fmt++;

				if (*fmt == '%') {
					putchar ('%');
					break;
				} else if (*fmt == 'b') {
					char *p = getstr();
					if (print_escape_str(p)) {
						return (rval);
					}
					break;
				}

				/* skip to field width */
				for (; strchr(SKIP1, *fmt); ++fmt)
					;
				if (*fmt == '*') {
					++fmt;
					havefieldwidth = 1;
					fieldwidth = getint();
				} else
					havefieldwidth = 0;

				/* skip to field precision */
				for (; strchr(SKIP2, *fmt); ++fmt)
					;
				haveprecision = 0;
				if (*fmt == '.') {
					++fmt;
					if (*fmt == '*') {
						++fmt;
						haveprecision = 1;
						precision = getint();
					}
					for (; strchr(SKIP2, *fmt); ++fmt)
						;
				}

				if (!*fmt) {
					warnx ("missing format character");
					return(1);
				}

				convch = *fmt;
				nextch = *(fmt + 1);
				*(fmt + 1) = '\0';
				switch(convch) {
				case 'c': {
					char p = getchr();
					PF(start, p);
					break;
				}
				case 's': {
					char *p = getstr();
					PF(start, p);
					break;
				}
				case 'd':
				case 'i': {
					long p;
					char *f = mklong(start, convch);
					if (!f) {
						warnx("out of memory");
						return (1);
					}
					p = getlong();
					PF(f, p);
					break;
				}
				case 'o':
				case 'u':
				case 'x':
				case 'X': {
					unsigned long p;
					char *f = mklong(start, convch);
					if (!f) {
						warnx("out of memory");
						return (1);
					}
					p = getulong();
					PF(f, p);
					break;
				}
				case 'a':
				case 'A':
				case 'e':
				case 'E':
				case 'f':
				case 'F':
				case 'g':
				case 'G': {
					double p = getdouble();
					PF(start, p);
					break;
				}
				default:
					warnx ("%s: invalid directive", start);
					return(1);
				}
				*(fmt + 1) = nextch;
				break;

			case '\\':
				fmt += print_escape(fmt);
				break;

			default:
				putchar (*fmt);
				break;
			}
		}
	} while (gargv > argv && *gargv);

	return (rval);
}


/*
 * Print SysV echo(1) style escape string 
 *	Halts processing string and returns 1 if a \c escape is encountered.
 */
static int
print_escape_str(const char *str)
{
	int value;
	int c;

	while (*str) {
		if (*str == '\\') {
			str++;
			/* 
			 * %b string octal constants are not like those in C.
			 * They start with a \0, and are followed by 0, 1, 2, 
			 * or 3 octal digits. 
			 */
			if (*str == '0') {
				str++;
				for (c = 3, value = 0; c-- && isodigit(*str); str++) {
					value <<= 3;
					value += octtobin(*str);
				}
				putchar (value);
				str--;
			} else if (*str == 'c') {
				return 1;
			} else {
				str--;			
				str += print_escape(str);
			}
		} else {
			putchar (*str);
		}
		str++;
	}

	return 0;
}
```

Please translate translate Part 1 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
