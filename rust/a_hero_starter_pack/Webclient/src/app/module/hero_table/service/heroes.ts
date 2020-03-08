import {Injectable} from "@angular/core";
import {APIService} from "../../../service/api";

@Injectable({
    providedIn: "root",
})
export class HeroesService {
    private static readonly URL_HEROES: string = "/heroes";

    constructor(
        private apiService: APIService
    ) {}

    search_heroes(filter: any, on_success: any, on_failure: any): void {
        this.apiService.post(HeroesService.URL_HEROES, filter,
            (result) => on_success.call(on_success, result), on_failure);
    }
}
