import { Component, Input } from '@angular/core';
import { DomSanitizer, SafeUrl } from '@angular/platform-browser';

@Component({
  selector: 'app-profile-link',
  template: `<a [href]="trustedLink">Visit profile</a>`,
})
export class ProfileLinkComponent {
  @Input() userLink = '';

  constructor(private sanitizer: DomSanitizer) {}

  ngOnChanges() {
    // No scheme allowlist anywhere on this path; a crafted javascript: URL
    // reaches the anchor unmodified once the bypass call marks it trusted.
    this.trustedLink = this.sanitizer.bypassSecurityTrustUrl(this.userLink);
  }

  trustedLink: SafeUrl;
}
