use crate::objects::orb::Orb;
// ok let me explain
// first of all, each "Object" needs to have an update and draw function. this is to keep things
// accurate to the original cart.
//
// you would think to use dyn traits for this, but that's actually not possible because safely
// downcasting would be impossible
//
// so each "Object" holds a function pointer, and a string of that name, used for formatting
//
// this is normally perfectly fine, but serde can't handle function pointers
// giving rise to this
//
// i could use rust macros, but for a project like this i didn't feel like learning them
// here's the javascript i used to generate it
//
//
// console.log(["","Balloon","BigChest","Chest","FakeWall","FallFloor","Flag","FlyFruit","Fruit","Key","LifeUp","Messsage","Orb","Platform","Player","PlayerSpawn","RoomTitle","Smoke","Spring"].reduce((p,n)=>p+`
// ObjectType::${n}(_)=>{
//	obj.draw = ObjFunc(${n}::draw);
//	obj.update = ObjFunc(${n}::update);
//  obj.name = "${n}"
// }
// `))
//
//
//
use crate::objects::{
    balloon::Balloon, bigchest::BigChest, chest::Chest, fakewall::FakeWall, fallfloor::FallFloor,
    flag::Flag, flyfruit::FlyFruit, fruit::Fruit, key::Key, lifeup::LifeUp, message::Message,
    platform::Platform, player::Player, playerspawn::PlayerSpawn, roomtitle::RoomTitle,
    smoke::Smoke, spring::Spring,
};

use crate::structures::{ObjFunc, Object, ObjectType};
pub fn assign_skipped_consts(obj: &mut Object) {
    match obj.obj_type {
        ObjectType::Balloon(_) => {
            obj.draw = ObjFunc(Balloon::draw);
            obj.update = ObjFunc(Balloon::update);
            obj.name = "Balloon"
        }

        ObjectType::BigChest(_) => {
            obj.draw = ObjFunc(BigChest::draw);
            obj.update = ObjFunc(BigChest::update);
            obj.name = "BigChest"
        }

        ObjectType::Chest(_) => {
            obj.draw = ObjFunc(Chest::draw);
            obj.update = ObjFunc(Chest::update);
            obj.name = "Chest"
        }

        ObjectType::FakeWall(_) => {
            obj.draw = ObjFunc(FakeWall::draw);
            obj.update = ObjFunc(FakeWall::update);
            obj.name = "FakeWall"
        }

        ObjectType::FallFloor(_) => {
            obj.draw = ObjFunc(FallFloor::draw);
            obj.update = ObjFunc(FallFloor::update);
            obj.name = "FallFloor"
        }

        ObjectType::Flag(_) => {
            obj.draw = ObjFunc(Flag::draw);
            obj.update = ObjFunc(Flag::update);
            obj.name = "Flag"
        }

        ObjectType::FlyFruit(_) => {
            obj.draw = ObjFunc(FlyFruit::draw);
            obj.update = ObjFunc(FlyFruit::update);
            obj.name = "FlyFruit"
        }

        ObjectType::Fruit(_) => {
            obj.draw = ObjFunc(Fruit::draw);
            obj.update = ObjFunc(Fruit::update);
            obj.name = "Fruit"
        }

        ObjectType::Key(_) => {
            obj.draw = ObjFunc(Key::draw);
            obj.update = ObjFunc(Key::update);
            obj.name = "Key"
        }

        ObjectType::LifeUp(_) => {
            obj.draw = ObjFunc(LifeUp::draw);
            obj.update = ObjFunc(LifeUp::update);
            obj.name = "LifeUp"
        }

        ObjectType::Message(_) => {
            obj.draw = ObjFunc(Message::draw);
            obj.update = ObjFunc(Message::update);
            obj.name = "Messsage"
        }

        ObjectType::Orb(_) => {
            obj.draw = ObjFunc(Orb::draw);
            obj.update = ObjFunc(Orb::update);
            obj.name = "Orb"
        }

        ObjectType::Platform(_) => {
            obj.draw = ObjFunc(Platform::draw);
            obj.update = ObjFunc(Platform::update);
            obj.name = "Platform"
        }

        ObjectType::Player(_) => {
            obj.draw = ObjFunc(Player::draw);
            obj.update = ObjFunc(Player::update);
            obj.name = "Player"
        }

        ObjectType::PlayerSpawn(_) => {
            obj.draw = ObjFunc(PlayerSpawn::draw);
            obj.update = ObjFunc(PlayerSpawn::update);
            obj.name = "PlayerSpawn"
        }

        ObjectType::RoomTitle(_) => {
            obj.draw = ObjFunc(RoomTitle::draw);
            obj.update = ObjFunc(RoomTitle::update);
            obj.name = "RoomTitle"
        }

        ObjectType::Smoke(_) => {
            obj.draw = ObjFunc(Smoke::draw);
            obj.update = ObjFunc(Smoke::update);
            obj.name = "Smoke"
        }

        ObjectType::Spring(_) => {
            obj.draw = ObjFunc(Spring::draw);
            obj.update = ObjFunc(Spring::update);
            obj.name = "Spring"
        }
    }
}
