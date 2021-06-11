# yts

**yts** lists yts.mx movies. It is a cli tool wrapped around a rust lib wrapped around their sweet [exposed api](https://yts.mx/api).

## Installation

First make sure you have the package manager [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) and then

```bash
cargo install yts
```

## Usage

```bash
yts help list
```

```
USAGE:
    yts list [FLAGS] [OPTIONS] [search]...

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information

        --rt
            get rotten tomatoes ratings


OPTIONS:
    -g, --genre <genre>
            Filter by a given genre

    -l, --limit <limit>
            The limit of results per page that has been set [default: 50]

        --mirror <mirror>
            domain / mirror to use [default: yts.mx]  [possible values: yts.mx, yts.lt, yts.am, yts.ag]

    -o, --order <order>
            Order the results ascending or descending [possible values: desc, asc]

    -p, --page <page>
            The page in the list of movies

    -q, --quality <quality>
            Filter by a given quality [possible values: 720p, 1080p, 2160p, 3D]

    -r, --rating <rating>
            Filter movie by a given minimum IMDb rating

    -s, --sort <sort>
            Sorts the results by a criteria [possible values: title, year, rating, peers, seeds, download_count, like_count, date_added]


ARGS:
    <search>...
            Search query
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)