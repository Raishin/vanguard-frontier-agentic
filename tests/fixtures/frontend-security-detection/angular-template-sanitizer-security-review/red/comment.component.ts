import { Component, Input } from '@angular/core';
import { DomSanitizer, SafeHtml } from '@angular/platform-browser';

@Component({
  selector: 'app-comment',
  template: `<div [innerHTML]="trustedComment"></div>`,
})
export class CommentComponent {
  @Input() userComment = '';
  trustedComment: SafeHtml;

  constructor(private sanitizer: DomSanitizer) {}

  ngOnChanges() {
    // Bypasses Angular's XSS sanitizer entirely; userComment flows straight
    // from a profile API that echoes attacker-submitted text.
    this.trustedComment = this.sanitizer.bypassSecurityTrustHtml(this.userComment);
  }
}
