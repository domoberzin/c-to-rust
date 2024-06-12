
It looks good to me. Now let us look at the next part of the program:

### Part 5

```c
char *
url_get_port (char *url) {
  char *port = malloc(sizeof(char));
  char *hostname = url_get_hostname(url);
  char *host = url_get_host(url);
  if (!port || !hostname) return NULL;

  char *tmp_hostname = strff(hostname, strlen(host) +1);
  sscanf(tmp_hostname, "%s", port);

  free(hostname);
  free(tmp_hostname);
  return port;
}

void
url_inspect (char *url) {
  url_data_inspect(url_parse(url));
}


void
url_data_inspect (url_data_t *data) {
  printf("#url =>\n");
  printf("    .href: \"%s\"\n",     data->href);
  printf("    .protocol: \"%s\"\n", data->protocol);
  printf("    .host: \"%s\"\n",     data->host);
  printf("    .auth: \"%s\"\n",     data->auth);
  printf("    .hostname: \"%s\"\n", data->hostname);
  printf("    .pathname: \"%s\"\n", data->pathname);
  printf("    .search: \"%s\"\n",   data->search);
  printf("    .path: \"%s\"\n",     data->path);
  printf("    .hash: \"%s\"\n",     data->hash);
  printf("    .query: \"%s\"\n",    data->query);
  printf("    .port: \"%s\"\n",     data->port);
}

void
url_free (url_data_t *data) {
  if (!data) return;
  if (data->auth) free(data->auth);
  if (data->protocol) free(data->protocol);
  if (data->hostname) free(data->hostname);
  if (data->host) free(data->host);
  if (data->pathname) free(data->pathname);
  if (data->path) free(data->path);
  if (data->hash) free(data->hash);
  if (data->search) free(data->search);
  if (data->query) free(data->query);
}
```

Please translate translate Part 5 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
