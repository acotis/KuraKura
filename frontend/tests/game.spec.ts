import { test, expect } from '@playwright/test';

test('Game component renders', async ({ page }) => {
  await page.goto('/');

  // Check that the page title exists
  await expect(page.getByRole('heading', { name: /kurakura lab/i })).toBeVisible();

  // Check that the game board renders (looking for the game container)
  await expect(page.locator('.flex.flex-col.border.p-4.gap-2')).toBeVisible();
});

test('One player can invite another player', async ({ browser }) => {
  // Create two browser contexts (two separate players) with clipboard permissions
  const hostContext = await browser.newContext({
    permissions: ['clipboard-read', 'clipboard-write'],
  });
  const guestContext = await browser.newContext();

  const hostPage = await hostContext.newPage();
  const guestPage = await guestContext.newPage();

  try {
    // Host player creates a game
    await hostPage.goto('/');
    await expect(hostPage.getByRole('heading', { name: /kurakura lab/i })).toBeVisible();

    // Wait for the "Copy Invite Link" button to appear (means room is created)
    const inviteButton = hostPage.getByRole('button', { name: /copy invite link/i });
    await expect(inviteButton).toBeVisible({ timeout: 10000 });

    // Click the invite button to copy the link
    await inviteButton.click();

    // Wait a bit for clipboard operation
    await hostPage.waitForTimeout(500);

    // Read the invite URL from the clipboard
    const inviteUrl = await hostPage.evaluate(() => {
      return navigator.clipboard.readText();
    });

    // Verify we got a join URL
    expect(inviteUrl).toContain('?join=');
    expect(inviteUrl.length).toBeGreaterThan(0);

    // Guest player joins using the invite link
    await guestPage.goto(inviteUrl);
    await expect(guestPage.getByRole('heading', { name: /kurakura lab/i })).toBeVisible();

    // Wait for WebSocket to connect and join room
    await guestPage.waitForTimeout(2000);

    // Verify guest successfully joined by checking they don't see "Waiting for player"
    await expect(guestPage.getByText(/waiting for player/i)).toBeHidden();

    // Verify guest sees the game board (there are multiple "Laqme" texts, just check the first one)
    await expect(guestPage.getByText('Laqme').first()).toBeVisible();

    // Note: The host doesn't receive a notification when guest joins (backend limitation),
    // so the invite button will still be visible on the host page. We can verify the
    // join was successful by checking the guest's state instead.

    // Verify the guest has both players in their game state
    const guestHasBothPlayers = await guestPage.evaluate(() => {
      const messagesDiv = document.querySelector('.text-xs');
      if (messagesDiv) {
        const text = messagesDiv.textContent || '';
        return text.includes('"players_present":2');
      }
      return false;
    });
    expect(guestHasBothPlayers).toBe(true);

  } finally {
    // Clean up
    await hostPage.close();
    await guestPage.close();
    await hostContext.close();
    await guestContext.close();
  }
});
