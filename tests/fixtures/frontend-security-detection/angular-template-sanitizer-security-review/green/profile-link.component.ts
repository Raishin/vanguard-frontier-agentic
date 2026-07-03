import { Component, Input } from '@angular/core';

@Component({
  selector: 'app-profile-link',
  // Scheme allowlist runs first; the template only ever binds to the
  // validated result or null, never to a raw bypass call.
  template: `<a [href]="isValidHttpUrl(userLink) ? userLink : null">Visit profile</a>`,
})
export class ProfileLinkComponent {
  @Input() userLink = '';

  isValidHttpUrl(value: string): boolean {
    try {
      const parsed = new URL(value);
      return parsed.protocol === 'http:' || parsed.protocol === 'https:';
    } catch {
      return false;
    }
  }
}
