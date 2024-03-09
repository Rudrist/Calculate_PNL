
//trait  SubAccount;
pub trait  SubAccount {
    fn update(&self, _child: &str, _id: i32, _tim: usize, mark_price: i32) {
        // let n;
        // let arr: Vec<CalPnlAlg>;

        // if _child == "portfolio" || _child == "quotation" || _child == "position" {
        //     arr = data_helper.get_data(_child, _id, _tim);
        //     n = arr.len();
        //     let mut alg = CalPnlAlg::new();

        //     for i in 0..n {
        //         alg.incoming_pnl += arr[i].incoming_pnl;
        //         alg.trading_pnl += arr[i].trading_pnl;
        //         alg.optimistic_pnl += arr[i].optimistic_pnl;
        //         alg.realized_pnl += arr[i].realized_pnl;
        //         alg.unrealized_pnl += arr[i].unrealized_pnl;
        //     }

        //     alg.net_pnl = alg.incoming_pnl + alg.trading_pnl;
        //     alg.total_pnl = alg.realized_pnl + alg.unrealized_pnl;

        //     data_helper.put_data(_child, _id, alg);
        // } else if _child == "order" {
        //     arr = data_helper.get_data(_child, _id, _tim);
        //     n = arr.len();
        //     let mut alg = data_helper.get_data("position", _id); // initialize a pnl constructor of position
        //     let mut sell = 0;

        //     for i in 0..n {
        //         if arr[i].state == OrderState::Pending {
        //             alg.optimistic_amount += if arr[i].side { arr[i].amount } else { -arr[i].amount };
        //             alg.optimistic_profit += if arr[i].side { arr[i].profit } else { -arr[i].profit };
        //         } else if arr[i].state == OrderState::Failed {
        //             alg.optimistic_amount -= if arr[i].side { arr[i].amount } else { -arr[i].amount };
        //             alg.optimistic_profit -= if arr[i].side { arr[i].profit } else { -arr[i].profit };
        //         } else if arr[i].state == OrderState::Opened {
        //             alg.trading_amount += if arr[i].side { arr[i].amount } else { -arr[i].amount };
        //             alg.trading_profit += if arr[i].side { arr[i].profit } else { -arr[i].profit };
        //             alg.unrealized_amount += if arr[i].side { arr[i].amount } else { -arr[i].amount };
        //             alg.unrealized_profit += if arr[i].side { arr[i].profit } else { -arr[i].profit };

        //             if !arr[i].side {
        //                 sell += arr[i].amount;
        //             }
        //         }
        //     }

        //     alg.optimistic_pnl = mark_price * alg.optimistic_amount - alg.optimistic_profit;
        //     alg.trading_pnl = mark_price * alg.trading_amount - alg.trading_profit;
        //     alg.net_pnl = alg.incoming_pnl + alg.trading_pnl;

        //     // Calculate FIFO
        //     const K: usize = 7; // 單位時間
        //     let mut fifo_time = data_helper.get_data(_child, _id, alg.fifo_order_id).time_stamp;

        //     while sell > 0 {
        //         arr = data_helper.get_data(_child, _id, fifo_time + K); // get order which state = opened  &&  side = 0
        //         n = arr.len();
        //         fifo_time += K;

        //         for i in 0..n {
        //             let dif = arr[i].amount - arr[i].sell;

        //             if dif == 0 {
        //                 continue;
        //             }

        //             if dif < sell {
        //                 alg.realized_amount += dif;
        //                 alg.realized_profit += dif * arr[i].price;
        //                 alg.unrealized_amount -= dif;
        //                 alg.unrealized_profit -= dif * arr[i].price;
        //                 arr[i].sell = arr[i].amount;
        //                 sell -= dif;
        //             } else {
        //                 alg.realized_amount += sell;
        //                 alg.realized_profit += sell * arr[i].price;
        //                 alg.unrealized_amount -= dif;
        //                 alg.unrealized_profit -= sell * arr[i].price;
        //                 arr[i].sell += sell;
        //                 sell = 0;
        //                 alg.fifo_order_id = arr[i].id;
        //                 break;
        //             }
        //         }
        //     }

        //     alg.realized_pnl = mark_price * alg.realized_amount - alg.realized_profit;
        //     alg.unrealized_pnl = mark_price * alg.unrealized_amount - alg.unrealized_profit;
        //     alg.total_pnl = alg.realized_pnl + alg.unrealized_pnl;

        //     // --------------------------------------------------------------------------------------------------------
        //     // Calculate LIFO    未優化的版本
        //     // const K: usize = 7;
        //     // let mut lifo_time = get_current_time();

        //     // while sell > 0 {
        //     //     arr = data_helper.get_data(_child, _id, lifo_time - K);
        //     //     n = arr.len();
        //     //     lifo_time -= K;

        //     //     for i in (0..n).rev() {
        //     //         let dif = arr[i].amount - arr[i].sell;

        //     //         if dif == 0 {
        //     //             continue;
        //     //         }

        //     //         if dif < sell {
        //     //             alg.realized_amount += dif;
        //     //             alg.realized_profit += dif * arr[i].price;
        //     //             alg.unrealized_amount -= dif;
        //     //             alg.unrealized_profit -= dif * arr[i].price;
        //     //             arr[i].sell = arr[i].amount;
        //     //             sell -= dif;
        //     //         } else {
        //     //             alg.realized_amount += sell;
        //     //             alg.realized_profit += sell * arr[i].price;
        //     //             alg.unrealized_amount -= dif;
        //     //             alg.unrealized_profit -= sell * arr[i].price;
        //     //             arr[i].sell += sell;
        //     //             sell = 0;
        //     //             alg.lifo_order_id = arr[i].id;
        //     //             break;
        //     //         }
        //     //     }
        //     // }

        //     // alg.realized_pnl = mark_price * alg.realized_amount - alg.realized_profit;
        //     // alg.unrealized_pnl = mark_price * alg.unrealized_amount - alg.unrealized_profit;
        //     // alg.total_pnl = alg.realized_pnl + alg.unrealized_pnl;

        //     data_helper.put_data(_id, alg);
        // }
    }

}
