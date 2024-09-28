let message = "Hello"
let hello () = print_endline message
let goodbye () = print_endline "Goodbye"

let hello_goodbye () =
  hello ();
  goodbye ()
