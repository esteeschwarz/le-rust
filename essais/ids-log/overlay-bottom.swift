//
//  overlaybottom.swift
//  MyProject
//
//  Designed in DetailsPro
//  Copyright © (My Organization). All rights reserved.
//

import SwiftUI

struct overlaybottom: View {
    var body: some View {
		ScrollView {
			VStack(spacing: 0) {
				Image("myImage")
					.renderingMode(.original)
					.resizable()
					.aspectRatio(contentMode: .fit)
				Image("myImage")
					.renderingMode(.original)
					.resizable()
					.aspectRatio(contentMode: .fit)
			}
			.frame(maxWidth: .infinity)
			.clipped()
			.padding(.top, 53)
			.padding(.bottom, 123)
		}
		.overlay(alignment: .bottom) {
			// Safari Bar
			VStack(spacing: 0) {
				Divider()
					.padding(.bottom, 8)
				// Address Bar
				HStack {
					Image(systemName: "textformat.size")
					Spacer()
					HStack(spacing: 2) {
						Image(systemName: "lock.fill")
							.imageScale(.small)
							.foregroundStyle(.secondary)
						Text("aimeleondore.com")
							.padding(.vertical, 11)
					}
					.font(.subheadline)
					Spacer()
					Image(systemName: "arrow.clockwise")
				}
				.padding(.horizontal, 13)
				.background {
					RoundedRectangle(cornerRadius: 12, style: .continuous)
						.fill(Color(.systemBackground))
						.shadow(color: .black.opacity(0.09), radius: 9, x: 0, y: 2)
						.shadow(color: .black.opacity(0.05), radius: 1, x: 0, y: 1)
				}
				.padding(.horizontal)
				.padding(.bottom, 10)
				.font(.subheadline)
				// Buttons
				HStack {
					Image(systemName: "chevron.backward")
					Image(systemName: "chevron.forward")
						.frame(maxWidth: .infinity)
						.clipped()
						.foregroundStyle(.secondary)
					Image(systemName: "square.and.arrow.up")
					Image(systemName: "book")
						.frame(maxWidth: .infinity)
						.clipped()
					Image(systemName: "checkmark.rectangle")
						.symbolRenderingMode(.monochrome)
						.foregroundStyle(.blue)
				}
				.padding(.horizontal, 60)
				.font(.system(.title3, weight: .light))
				.foregroundStyle(.primary)
				.frame(maxWidth: .infinity, alignment: .center)
				.clipped()
			}
			.frame(height: 123, alignment: .top)
			.clipped()
			.background {
				Rectangle()
					.fill(.clear)
					.background(Material.bar)
			}
		}
		.overlay(alignment: .top) {
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
			.background(Color(.systemBackground))
		}
    }
}

#Preview {
    overlaybottom()
}