## Band or Label Pages

A band or label page takes the form of `https://{band}.bandcamp.com/music`. The
`music` path is important because sometimes the main page redirect the latest
album or something else.

Albums/singles are contained in a `<ol>` with the `id` of `#music-grid`. Each
item is a `.music-grid-item` and contains an `<a>` with a relative link to the
album/single.

```html
<ol id="music-grid">
  <li class="music-grid-item">
    <a href="/album/foo">
      ...
    </a>
  </li>
</ol>
```

If a page has more than 16 albums/singles, only the first 16 are rendered
server-side and `#music-grid` will have a `data-client-items` attribute containing JSON data for all of the items that is used to populate the page dynamically client-side.

```json
[
  {
    "page_url": "/album/foo",
    ...
  },
  ...
]
```
