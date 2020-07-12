import { RoomplaApi } from "./apis";
import { Configuration } from "./runtime";
import { use } from "vue/types/umd";

export var store = {
    state: {
        api: new RoomplaApi(),
        userId: "",
    },
    login(token: string, userId: string) {
        this.state.api = new RoomplaApi(new Configuration({ accessToken: token }));
        this.state.userId = userId;
    },
    logout() {
        this.state.api = new RoomplaApi();
        this.state.userId = "";
    }
}