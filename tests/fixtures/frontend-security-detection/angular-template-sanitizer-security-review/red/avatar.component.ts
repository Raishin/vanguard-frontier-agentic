import { Component, Input } from '@angular/core';
import { DomSanitizer, SafeResourceUrl } from '@angular/platform-browser';

@Component({
  selector: 'app-avatar',
  template: `<img [src]="trustedImage" />`,
})
export class AvatarComponent {
  @Input() userImage = '';
  trustedImage: SafeResourceUrl;

  constructor(private sanitizer: DomSanitizer) {}

  ngOnChanges() {
    // Marks an arbitrary user-supplied resource URL as trusted with no
    // scheme or origin validation on the path.
    this.trustedImage = this.sanitizer.bypassSecurityTrustResourceUrl(this.userImage);
  }
}
