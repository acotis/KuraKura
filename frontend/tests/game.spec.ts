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

    // Now test that one player can make a move and the other sees it
    // Black player (host) goes first
    console.log('Testing move submission and synchronization...');

    // Host places a stone at position (2, 2)
    const hostBoard = hostPage.locator('table tbody');
    const hostCell = hostBoard.locator('tr').nth(2).locator('td').nth(2);
    await hostCell.click();

    // Wait for the spin selection overlay to appear
    await hostPage.waitForTimeout(500);

    // Select a 2x2 spin region by dragging from (1,1) to (2,2)
    const spinOverlay = hostPage.locator('div.absolute.inset-0');
    await spinOverlay.dispatchEvent('mousedown', {
      clientX: 1.5 * 40 + 32, // tileSize is 40, offset by 32px padding
      clientY: 1.5 * 40 + 32,
    });
    await spinOverlay.dispatchEvent('mousemove', {
      clientX: 2.5 * 40 + 32,
      clientY: 2.5 * 40 + 32,
    });
    await spinOverlay.dispatchEvent('mouseup');

    // Wait for the confirm button to appear
    await hostPage.waitForTimeout(500);

    // Click the checkmark button to confirm the move
    const confirmButton = hostPage.locator('button:has-text("✓")');
    await expect(confirmButton).toBeVisible();
    await confirmButton.click();

    // Wait for the move to be sent and processed
    await hostPage.waitForTimeout(1000);
    await guestPage.waitForTimeout(1000);

    // Verify both players see the stone on the board
    // Check that there's a stone visible at position (2, 2) on both boards
    const hostStoneExists = await hostPage.evaluate(() => {
      const table = document.querySelector('table tbody');
      if (!table) return false;
      const rows = table.querySelectorAll('tr');
      if (rows.length < 3) return false;
      const cells = rows[2].querySelectorAll('td');
      if (cells.length < 3) return false;
      // Check if there's stone content (looking for the stone element)
      const cellContent = cells[2].textContent || '';
      return cells[2].querySelector('[class*="stone"]') !== null || cellContent.includes('1');
    });

    const guestStoneExists = await guestPage.evaluate(() => {
      const table = document.querySelector('table tbody');
      if (!table) return false;
      const rows = table.querySelectorAll('tr');
      if (rows.length < 3) return false;
      const cells = rows[2].querySelectorAll('td');
      if (cells.length < 3) return false;
      const cellContent = cells[2].textContent || '';
      return cells[2].querySelector('[class*="stone"]') !== null || cellContent.includes('1');
    });

    console.log('Host sees stone:', hostStoneExists);
    console.log('Guest sees stone:', guestStoneExists);

    expect(hostStoneExists).toBe(true);
    expect(guestStoneExists).toBe(true);

  } finally {
    // Clean up
    await hostPage.close();
    await guestPage.close();
    await hostContext.close();
    await guestContext.close();
  }
});
