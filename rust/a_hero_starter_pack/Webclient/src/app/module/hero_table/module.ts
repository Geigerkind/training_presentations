import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {HeroTableComponent} from "./component/hero_table/hero_table";
import {CommonModule} from "@angular/common";
import { TableModule } from 'src/app/template/table/module';

@NgModule({
    declarations: [HeroTableComponent],
    imports: [
        CommonModule,
        TranslateModule,
        TableModule
    ],
    exports: [HeroTableComponent]
})
export class HeroTableModule {
}
