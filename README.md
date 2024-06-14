# vroom
Like [`boom`](https://github.com/holman/boom) but faster. Vrooooom!

## Example
```
$ vroom
bookmarks (3)
gh_projects (3)

$ vroom bookmarks
google: https://google.com
github: https://github.com
reddit: https://reddit.com

$ vroom delete bookmarks reddit
Deleting 'reddit' from 'bookmarks'

$ vroom bookmarks lemmy https://lemmy.ml
'lemmy' in 'bookmarks' is 'https://lemmy.ml'

$ vroom bookmarks
google: https://google.com
github: https://github.com
lemmy:  https://lemmy.ml

$ firefox "$(vroom lemmy)"
```

## Full List of Commands
- Overview:
```
$ vroom
bookmarks (2)
gh_projects (3)
```
- Create a list: `vroom <list-name>`
- Add an item: `vroom <list-name> <item-name> <value>`
- Print an item: `vroom <item-name>` or `vroom <list-name> <item-name>`
- Get all items in a list: `vroom <list-name>`
```
$ vroom bookmarks
google: https://google.com
github: https://github.com
```
- Delete a list: `vroom delete <list-name>`
- Delete an item: `vroom delete <list-name> <item-name>`
- List Everything: `vroom all`
```
$ vroom all
bookmarks
  google: https://google.com
  github: https://github.com
  lemmy:  https://lemmy.ml
gh_projects
  boom:  https://github.com/holman/boom
  dots:  https://github.com/bluedragon1221/dotfiles
  helix: https://github.com/helix-editor/helix
  vroom: https://github.com/bluedragon1221/vroom
```
## Examples
Open in browser:
```
firefox "$(vroom github)/bluedragon1221"
```

Directory bookmarks:
```
$ vroom dirs
cfg: ~/.config
...

$ cd $(vroom cfg)/zsh
```

Copy to clipboard:
```
$ vroom thoughts
robot: What if we're all robots?

$ vroom robot | wl-copy
```

Find that one thing you can't remember:
```
vroom all | grep "robots"
  robot: What if we're all robots?
```
