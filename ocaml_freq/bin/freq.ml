let regex = Str.regexp {|\([A-Za-z0-9-_]+\)|}

let match_regex str ~start =
  try
    let _ = Str.search_forward regex str start in
    Some (Str.matched_string str, Str.match_end ())
  with Not_found -> None

let reduce_matches f init str =
  let rec aux f acc str start =
    match match_regex str ~start with
    | None -> acc
    | Some (substring, new_start) -> aux f (f acc substring) str new_start
  in
  aux f init str 0

let make_word_freq_map acc line =
  let f acc str = Counter.touch str acc in
  reduce_matches f acc line

let make_freq_map_from_stdin () =
  let rec aux acc =
    match In_channel.input_line In_channel.stdin with
    | None -> acc
    | Some line -> aux (make_word_freq_map acc line)
  in
  aux Counter.empty

let () =
  make_freq_map_from_stdin ()
  |> Counter.to_list
  |> List.iter (fun (str, count) -> Printf.printf "%s: %d\n" str count)
