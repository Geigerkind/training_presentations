import {Component, OnInit} from "@angular/core";
import {HeaderColumn} from "../../../../template/table/module/table_header/domain_value/header_column";
import {BodyColumn} from "../../../../template/table/module/table_body/domain_value/body_column";
import { BodyRow } from 'src/app/template/table/module/table_body/domain_value/body_row';
import { SettingsService } from 'src/app/service/settings';
import { table_init_filter } from 'src/app/template/table/utility/table_init_filter';
import { HeroesService } from '../../service/heroes';


@Component({
    selector: "HeroTable",
    templateUrl: "./hero_table.html",
    styleUrls: ["./hero_table.scss"]
})
export class HeroTableComponent implements OnInit {

    header_columns: Array<HeaderColumn> = [
        {index: 0,filter_name: 'id',labelKey: "Heroes.id",type: 1,type_range: null,col_type: 3},
        {index: 1, filter_name: 'name', labelKey: "Heroes.name", type: 0, type_range: null, col_type: 0},
        {index: 2, filter_name: 'sub_title', labelKey: "Heroes.sub_title", type: 0, type_range: null, col_type: 0},
        {index: 3, filter_name: 'strength', labelKey: "Heroes.strength", type: 0, type_range: null, col_type: 0},
        {index: 4, filter_name: 'weakness', labelKey: "Heroes.weakness", type: 0, type_range: null, col_type: 0},
        {index: 5, filter_name: 'hero_call', labelKey: "Heroes.hero_call", type: 0, type_range: null, col_type: 0},
    ];
    body_columns: Array<BodyRow> = [];
    clientSide: boolean = false;
    responsiveHeadColumns: Array<number> = [0, 1];
    responsiveModeWidthInPx: number = 840;
    num_characters: number = 0;

    constructor(
        private heroesService: HeroesService,
        private settingsService: SettingsService
    ) {
        this.filterHeroes(this.settingsService.get("table_filter_heroes_search"));
    }

    ngOnInit(): void {
        const filter = table_init_filter(this.header_columns);
        if (!this.settingsService.check("table_filter_heroes_search")) {
            this.settingsService.set("table_filter_heroes_search", filter);
        }
    }

    filterHeroes(filter: any): void {
        this.heroesService.search_heroes(filter,
            (search_result) => {
                this.num_characters = search_result.num_items;
                this.body_columns = search_result.result.map(row => {
                    const body_columns: Array<BodyColumn> = [];
                    body_columns.push({
                        type: 1,
                        content: row.id.to_string(),
                        args: null
                    });
                    body_columns.push({
                        type: 0,
                        content: row.name,
                        args: null
                    });
                    body_columns.push({
                        type: 0,
                        content: row.sub_title ? row.sub_title : '',
                        args: null
                    });
                    body_columns.push({
                        type: 0,
                        content: row.strength,
                        args: null
                    });
                    body_columns.push({
                        type: 0,
                        content: row.weakness ? row.weakness : '',
                        args: null
                    });
                    body_columns.push({
                        type: 0,
                        content: row.hero_call ? row.hero_call : '',
                        args: null
                    });

                    return {
                        color: null,
                        columns: body_columns
                    };
                });
            },
            () => {
            });
    }

}
