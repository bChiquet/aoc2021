pub fn p1_1(vec: Vec<usize>) -> usize {
   let (_, sum) = vec.iter()
       .fold((None, 0), |state, item| match state {
            (None, _) =>
                (Some(item), 0),
            (Some(pred), acc) =>
                (Some(item), if item > pred {acc+1} else {acc})
       });
   sum
}
