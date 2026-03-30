# Web server
**Web server** manages the coordination between following components:
- `socket server`;
- `httplib`;
- `router`;
- `handlers`;

<br>

The `socket server` **receives byte stream** from client.<br>
The `httplib` **interprets byte stream** from **socket** and **converts** it to **HTTP request**.<br>
The `router` **accepts HTTP request** and **passes** it to appropriate `handler`.<br>
The `handler` processes the **HTTP request** and returns **HTTP response**.<br>
The `httplib` converts **HTTP response** back to **byte stream**.<br>
The `socket server` **sends byte stream** back to the client.<br>

<br>

The `handler` and `router` signatures: `Fn(HTTPrequest) -> HTTPresponse`.<br>
The `httplib` contains:
- `struct HTTPrequest`;
- `struct HTTPresponse`;
- `impl From<String> for HTTPrequest`;
- `impl From<HTTPresponse> for String`;

<br>

# Actix
The **actix web server** consists of:
- `HTTP server` (aka **actix web server**);
- `App`;
- `Routes`;

<br>

`HTTP server` is responsible for serving **HTTP requests**. It implements **HTTP protocol** parser.<br>
`App` (aka **actix web application**) is a **group** of some **routes**.<br>
`Route` in actix web is **3-tuple**: (`URI`, `HTTP method`, `Handler`).<br>

<br>

## Application state
An actix web server spawns threads on startup. Each thread can process **incoming requests** independently.<br>
All routes witihin `App` can share **application state**.<br>
**Application state** can be treated as some **data structure**.<br>
There is `web::Data` **extractor** to extract **application state** inside `handler`.<br>