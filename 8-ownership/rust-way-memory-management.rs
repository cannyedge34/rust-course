fn get_final_orders() -> i64 {
  let mut total_orders = 0

  // anonymous scope
  {
    let orders = vec![1, 2, 3] // alloc

    for order in orders.iter() {
      total_orders += orders;
    }
  } // dealloc(orders)

  return finish(total_orders)
}