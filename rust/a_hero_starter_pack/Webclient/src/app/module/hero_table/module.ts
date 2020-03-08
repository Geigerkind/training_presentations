import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {HeroTableComponent} from "./component/hero_table/hero_table";
import {CommonModule} from "@angular/common";

@NgModule({
    declarations: [HeroTableComponent],
    imports: [
        CommonModule,
        TranslateModule
    ],
    exports: [HeroTableComponent]
})
export class HeroTableModule {
}
