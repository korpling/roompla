import { RoomplaApi } from "./apis";
import { Configuration } from "./runtime";

export var store = {
    state: {
        api: new RoomplaApi(),
    },
    login(token: string) {
        this.state.api = new RoomplaApi(new Configuration({ accessToken: token }));
    },
    logout() {
        this.state.api = new RoomplaApi();
    }
}