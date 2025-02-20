<div align="center">
  <p>A simple terminal UI for making HTTP Request calls using Curl.</p>

<p>
<img src="https://github.com/Jorgexyx/lazycurl/assets/23204093/6f49db4e-e4cf-4d4b-a149-b8d9a1222998" alt="Lazy Curl Logo" width="500"/>
</p>

<br/>
<p>Alternative to Postman | inspired by lazygit</p>
<br />

<img width="1486" alt="Screenshot 2024-03-26 at 8 08 29 AM" src="https://github.com/Cypressxyx/lazycurl/assets/23204093/38d1efa8-e2ad-4b66-adf7-560a1cbb967a">

<br/>

## Why

Alternative to postman without all the  bloat

## Installation
```
$ cargo run
```

## KeyBindings

### Body

```
q - quit
Esc - deselect
h - select history window

1 - focus URL window
2 - focus Parameters window
3 - focus Response window

Enter - send request
```

### URL Window (1)

```
e - edit URL
Esc - submit URL
[ - POST
] - GET
```

### Parameters Window (2)

```
[ - Headers
] - Body

    # Headers
    j - traverse down request
    k - traverse up request
    e - edit mode
    a - add new header
    TAB - switch between key and value (edit mode)

    # Body
    e - edit mode

```
### Response Window (3)

    `...`

### History Window
```
    j - traverse down request
    k - traverse up request
    Space, l, Enter - send request
```
