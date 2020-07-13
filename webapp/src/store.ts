import { RoomplaApi } from "./apis";
import { Configuration } from "./runtime";
import { use } from "vue/types/umd";
import { Room } from "./models";

interface RoomMap {
    [id: string]: Room; 
}

export var store = {
    state: {
        api: new RoomplaApi(),
        userId: "",
        rooms: <RoomMap>{},
    },
    login(token: string, userId: string) {
        this.state.api = new RoomplaApi(new Configuration({ accessToken: token }));
        this.state.userId = userId;
    },
    logout() {
        this.state.api = new RoomplaApi();
        this.state.userId = "";
    },
    set_rooms(rooms : Array<Room>) {
        // Convert array to map
        this.state.rooms = {};
        for(const r of rooms) {
            if(r.id) {
                const id : string = r.id;
                this.state.rooms[id] = r;
            }
        }
    }
}