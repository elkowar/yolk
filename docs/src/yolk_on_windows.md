# Yolk on Windows

Yolk can be used on all platforms, and allows you to declare different file locations for your configuration files per platform.

However, there are a few special things to keep in mind when using yolk on windows.
Primarily, Windows does not allow users to create symlinks by default. As yolk relies on symlinks to deploy dotfiles, you will need to apply one of the following solutions to this problem:

1. [Activate developer mode](https://learn.microsoft.com/en-us/windows/apps/get-started/enable-your-device-for-development):
  Simply activate the Developer Mode in the windows settings menu. This is the easiest solution.
2. Always run yolk with administrator permissions. This is not recommended.
3. Explicitly allow creating symlinks for your user through the windows security policy.
