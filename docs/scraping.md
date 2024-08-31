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
server-side and `#music-grid` will have a `data-client-items` attribute
containing JSON data for all of the items that is used to populate the page
dynamically client-side.

```json
[
  {
    "page_url": "/album/foo",
    ...
  },
  ...
]
```
## Album Pages

Album pages take to form of `https://{band}.bandcamp.com/album/{album}`. There
is a `<script>` with an attribute `data-tralbum` that contains JSON data for
each track including a link to the MP3 file.

Note that on albums with multiple artists, the `artist` fields under `trackinfo`
might take the form of "Artist - Title".

```json
{
  "artist": "album artist",
  "album_release_date": "01 Jan 2000 00:00:00 GMT",
  "trackinfo": [
    {
      "file": {
        "mp3-128": "https://t4.bcbits.com/stream/deadbeef"
      },
      "artist": "track artist",
      "title": "track title",
      "track_num": 1,
      ...
    },
    ...
  ],
  ...
}
```
