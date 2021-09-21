```sh
var = http_client [--method method] [--payload payload] [--output-file file] URL
```

Invokes a HTTP request.<br>
The request method by default is GET but can be modified by the ```--method``` parameter.<br>
The ```--output-file``` parameter will redirect a valid response output to the provided file, otherwise all response text will be set to the
output variable.<br>
When redirecting to file, the output would be the response size.<br>
The ```--payload``` parameter enables to pass a payload to POST http requests.<br>
In case of errors or error HTTP response codes, false will be returned.

### Parameters

* Optional HTTP Method, for example ```--method GET``` or ```--method POST``` (currently only GET and POST are supported).
* Optional post payload via ```--payload``` parameter.
* Optional redirection of output to file via ```--output-file``` parameter.
* The target URL

### Return Value

The response text or in case of output redirection to file, the response size.<br>
In case of errors, it will return false.

### Examples

```sh
function test_get
    response = http_client https://www.rust-lang.org/

    found = contains ${response} Rust

    assert ${found}
end

function test_get_to_file
    file = set ./target/_duckscript_test/http_client/page.html
    rm ${file}

    response_size = http_client --output-file ${file} https://www.rust-lang.org/

    response = readfile ${file}
    found = contains ${response} Rust

    assert ${found}
    assert ${response_size}
end

function test_post
    payload = set {\"login\":\"login\",\"password\":\"password\"}
    response = http_client --method POST --payload ${payload} https://reqbin.com/echo/post/json

    found = contains ${response} success

    assert ${found}
end
```
