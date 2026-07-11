# Yolk on Windows

Yolk can be used on all platforms, and allows you to declare different file locations for your configuration files per platform.

However, there are a few special things to keep in mind when using Yolk on Windows.
Primarily, Windows does not allow users to create symlinks by default. As yolk relies on symlinks to deploy dotfiles, you will need to apply one of the following solutions to this problem:

1. [Activate Developer Mode](https://learn.microsoft.com/en-us/windows/apps/get-started/enable-your-device-for-development):
  Simply activate Developer Mode in the Windows Settings menu. This is the easiest solution.
2. Always run yolk with administrator permissions. This is not recommended.
3. Explicitly allow your user to create symlinks through the Windows security policy.
