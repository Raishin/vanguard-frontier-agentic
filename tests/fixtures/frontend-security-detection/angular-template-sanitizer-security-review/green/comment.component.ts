import { Component, Input } from '@angular/core';

@Component({
  selector: 'app-comment',
  // Text interpolation: Angular's updateTextNode uses renderer.setValue()
  // (the text content API), which never parses HTML, so no sanitizer
  // bypass is needed or possible here.
  template: `<div>{{ userComment }}</div>`,
})
export class CommentComponent {
  @Input() userComment = '';
}
