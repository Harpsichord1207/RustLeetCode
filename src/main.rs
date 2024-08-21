mod l126;
mod l306;
mod l368;
mod l438;
mod l463;
mod l541;
mod l543;
mod l796;
mod l855;
mod l859;
mod l997;
mod l1021;
mod l1054;
mod l1395;
mod l1756;
mod l2582;
mod l3238;

use l855::ExamRoom;

fn main() {
    let mut exam_room = ExamRoom::new(8);
    exam_room.seat();
    exam_room.seat();
    exam_room.seat();
    exam_room.leave(0);
    exam_room.leave(7);
    exam_room.seat();
    exam_room.seat();
    exam_room.seat();
    exam_room.seat();
    exam_room.seat();
    exam_room.seat();
    exam_room.seat();
    println!("{:?}", exam_room.occupied);
}
