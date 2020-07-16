import { RoomplaApi } from "./apis";
import { Configuration } from "./runtime";
import { use } from "vue/types/umd";
import { Room } from "./models";
import * as process from 'process'

interface RoomMap {
    [id: string]: Room;
}

export function getAPIBasePath() {
    if (process.env.NODE_ENV === 'development') {
        return "http://localhost:5050/roompla/v0";
    } else {
        return "/roompla/v0";
    }
}

export var store = {
    state: {
        api: new RoomplaApi(new Configuration({ basePath: getAPIBasePath() })),
        userId: "",
        rooms: <RoomMap>{},
    },
    login(token: string, userId: string) {

        this.state.api = new RoomplaApi(new Configuration({ accessToken: token, basePath: getAPIBasePath() }));
        this.state.userId = userId;
    },
    logout() {
        this.state.api = new RoomplaApi(new Configuration({ basePath: getAPIBasePath() }));
        this.state.userId = "";
    },
    set_rooms(rooms: Array<Room>) {
        // Convert array to map
        this.state.rooms = {};
        for (const r of rooms) {
            if (r.id) {
                const id: string = r.id;
                this.state.rooms[id] = r;
            }
        }
    }
}