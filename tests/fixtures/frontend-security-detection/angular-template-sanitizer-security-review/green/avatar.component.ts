import { Component, Input } from '@angular/core';
import { DomSanitizer, SecurityContext } from '@angular/platform-browser';

@Component({
  selector: 'app-avatar',
  // ngSrc requires a plain string, validated through DomSanitizer.sanitize()
  // against SecurityContext.URL rather than an unchecked bypass call.
  template: `<img [ngSrc]="safeImageUrl" width="64" height="64" />`,
})
export class AvatarComponent {
  @Input() userImage = '';
  safeImageUrl = '';

  constructor(private sanitizer: DomSanitizer) {}

  ngOnChanges() {
    this.safeImageUrl = this.sanitizer.sanitize(SecurityContext.URL, this.userImage) ?? '';
  }
}
