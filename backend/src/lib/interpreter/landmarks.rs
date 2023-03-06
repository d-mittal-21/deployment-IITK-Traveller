use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static!{
    pub static ref LANDMARKS: Mutex<HashMap<&'static str, i32>> = {
        let mut map = HashMap::new();
        map.insert("start", 0);     //start
        map.insert("finish", 1);    //finish
        map.insert("iit_gate_in_1", 2); //input to M[mem1]
        map.insert("iit_gate_in_2", 3); //input to M[mem2]
        map.insert("hall_2", 4);        // mem[mem_3] = mem[mem_1] + mem[mem_2]
        map.insert("hall_3", 5);        // mem[mem_3] = mem[mem_1] * mem[mem_2]
        map.insert("hall_5", 6);        // mem[mem_3] = mem[mem_1] - mem[mem_2]
        map.insert("hall_12", 7);       // mem[mem_3] = mem[mem_1] / mem[mem_2]
        map.insert("mt_1_3", 8);        // mem[mem_1] = mem[mem_3]
        map.insert("mt_3_1", 9);        // mem[mem_3] = mem[mem_1]
        map.insert("mt_2_3", 10);       // mem[mem_2] = mem[mem_3]
        map.insert("mt_3_2", 11);       // mem[mem_3] = mem[mem_2]
        map.insert("iit_gate_out_1", 12);   // output mem[mem_1]
        map.insert("iit_gate_out_2", 13);   // output mem[mem_2]
        map.insert("lecture_hall_gt", 14);  // mem[mem_1] > mem[mem_2]
        map.insert("lecture_hall_gt_t", 15);
        map.insert("lecture_hall_gt_f", 16);
        map.insert("lecture_hall_lt", 17);  // mem[mem_1] < mem[mem_2]
        map.insert("lecture_hall_lt_t", 18);
        map.insert("lecture_hall_lt_f", 19);
        map.insert("lecture_hall_eq", 20);  // mem[mem_1] == mem[mem_2]
        map.insert("lecture_hall_eq_t", 21);
        map.insert("lecture_hall_eq_f", 22);
        map.insert("oat_stairs_1", 23);     // mem[mem_1]++
        map.insert("oat_stairs_2", 24);     // mem[mem_2]++
        map.insert("oat_stairs_c", 25);     // cond++
        map.insert("southern_labs_1", 26);  // mem[mem_1]--;
        map.insert("southern_labs_2", 27);  // mem[mem_2]--;
        map.insert("southern_labs_c", 28);  // cond--
        map.insert("hall_13_1", 29);        // mem[mem_1] = 0;
        map.insert("hall_13_2", 30);        // mem[mem_2] = 0;
        map.insert("hall_13_3", 31);        // mem[mem_3] = 0;
        map.insert("hall_13_c", 32);        // cond = 0;
        map.insert("rm_1", 33);             // mem_1++
        map.insert("rm_2", 34);             // mem_2++
        map.insert("rm_3", 35);             // mem_3++
        map.insert("kd_1", 36);             // mem_1--
        map.insert("kd_2", 37);             // mem_2--
        map.insert("kd_3", 38);             // mem_3--
        Mutex::new(map)
    };
}