import {AfterViewInit, Component, OnInit} from '@angular/core';
import {CommonModule} from '@angular/common';
import {MatInputModule} from "@angular/material/input";
import {MarkdownComponent, MarkdownPipe, MarkdownService} from "ngx-markdown";
import {FormsModule} from "@angular/forms";
import {AngularMarkdownEditorModule, EditorOption} from "angular-markdown-editor";

@Component({
    selector: 'app-create-puzzle-set',
    standalone: true,
    imports: [CommonModule, MatInputModule, MarkdownComponent, FormsModule, AngularMarkdownEditorModule, MarkdownPipe],
    templateUrl: './create-puzzle-set.component.html',
    styleUrl: './create-puzzle-set.component.css'
})
export class CreatePuzzleSetComponent implements OnInit {

    constructor(private markdownService: MarkdownService) {
    }

    content: string = "";

    ngOnInit(): void {
        this.editorOptions = {
            disabledButtons: ["preview", "fullscreen", "guide", "image", "mode", "both"]
        }
    }

    protected readonly console = console;
    editorOptions: EditorOption | undefined;

    protected readonly document = document;
}
