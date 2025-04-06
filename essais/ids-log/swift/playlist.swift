//
//  playlist.swift
//  MyProject
//
//  Designed in DetailsPro
//  Copyright © (My Organization). All rights reserved.
//

import SwiftUI

struct playlist: View {
    var body: some View {
		ScrollView {
			VStack(spacing: 0) {
				Image("myImage")
					.renderingMode(.original)
					.resizable()
					.aspectRatio(contentMode: .fit)
					.frame(width: 250, height: 250)
					.clipped()
					.mask { RoundedRectangle(cornerRadius: 10, style: .continuous) }
					.shadow(color: .black.opacity(0.2), radius: 8, x: 0, y: 4)
					.padding(.top)
				Text("So Much Wine - EP")
					.font(.system(.headline, weight: .semibold))
					.padding(.top)
				Text("Phoebe Bridgers")
					.font(.system(.headline, weight: .regular))
					.foregroundStyle(.pink)
					.padding(.top, 2)
				Text("Alternative • 2022 • Lossless")
					.font(.system(.footnote, weight: .regular))
					.foregroundStyle(.secondary)
					.padding(.top, 2)
				// Buttons
				HStack(spacing: 20) {
					HStack {
						Image(systemName: "play.fill")
						Text("Play")
					}
					.foregroundStyle(.pink)
					.font(.system(.subheadline, weight: .semibold))
					.padding(.vertical, 12)
					.frame(maxWidth: .infinity)
					.clipped()
					.background {
						RoundedRectangle(cornerRadius: 10, style: .continuous)
							.fill(Color(.quaternarySystemFill))
					}
					HStack {
						Image(systemName: "shuffle")
						Text("Shuffle")
					}
					.foregroundStyle(.pink)
					.font(.system(.subheadline, weight: .semibold))
					.padding(.vertical, 12)
					.frame(maxWidth: .infinity)
					.clipped()
					.background {
						RoundedRectangle(cornerRadius: 10, style: .continuous)
							.fill(Color(.quaternarySystemFill))
					}
				}
				.padding()
				VStack(spacing: 0) {
					ForEach(0..<5) { _ in // Replace with your data model here
						Divider()
							.padding(.leading)
							.opacity(0.5)
						HStack(spacing: 0) {
							// Track #
							Text("1")
								.frame(width: 22, alignment: .leading)
								.clipped()
								.foregroundStyle(.secondary)
							// Track Name
							Text("So Much Wine")
							Spacer()
							Image(systemName: "ellipsis")
						}
						.padding(.horizontal)
						.padding(.vertical, 14)
						.font(.subheadline)
						.padding(.leading, 8)
					}
					Divider()
						.padding(.leading)
						.opacity(0.5)
				}
				Text("""
				November 17, 2022
				6 songs, 22 minutes
				2022 Dead Oceans
				""")
					.font(.footnote)
					.foregroundStyle(.secondary)
					.padding()
					.frame(maxWidth: .infinity, alignment: .leading)
					.clipped()
			}
			.frame(maxWidth: .infinity)
			.clipped()
			.padding(.top, 98)
			.padding(.bottom, 150)
		}
		.overlay(alignment: .top) {
			// Navigation Bar
			VStack(spacing: 1) {
				// Status Bar
				HStack {
					Text("9:41")
						.frame(width: 109)
						.clipped()
						.font(.system(.body, weight: .semibold))
					Spacer()
					HStack(spacing: 5) {
						Image(systemName: "cellularbars")
							.imageScale(.small)
						Image(systemName: "wifi")
							.imageScale(.small)
						Image(systemName: "battery.100")
							.symbolRenderingMode(.hierarchical)
							.font(.system(.body, weight: .light))
					}
					.frame(width: 109)
					.clipped()
					.font(.system(.body, weight: .semibold))
				}
				.padding(.horizontal)
				.padding(.top, 5)
				.frame(maxWidth: .infinity)
				.clipped()
				.frame(height: 53)
				.clipped()
				HStack(spacing: 0) {
					Text("So Much Wine - EP")
						.font(.headline)
				}
				.frame(height: 44)
				.clipped()
			}
			.frame(height: 98)
			.clipped()
			.background {
				Rectangle()
					.fill(.clear)
					.background(Material.bar)
			}
		}
		.overlay(alignment: .bottom) {
			// Tab Bar
			VStack(spacing: 0) {
				Divider()
				HStack(spacing: 10) {
					ForEach(0..<5) { _ in // Replace with your data model here
						VStack(spacing: 4) {
							Image(systemName: "play.circle.fill")
								.imageScale(.large)
								.frame(height: 26)
								.clipped()
							Text("Listen Now")
								.font(.caption2)
						}
						.frame(maxWidth: .infinity)
						.clipped()
						.frame(height: 45)
						.clipped()
						.foregroundStyle(.secondary)
					}
				}
				.padding(.horizontal, 15)
				.padding(.top, 5)
			}
			.frame(height: 84, alignment: .top)
			.clipped()
			.background {
				Rectangle()
					.fill(.clear)
					.background(Material.bar)
			}
		}
    }
}

#Preview {
    playlist()
}