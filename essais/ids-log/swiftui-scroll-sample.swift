//
//  swiftuiscrolltemplate.swift
//  MyProject
//
//  Designed in DetailsPro
//  Copyright © (My Organization). All rights reserved.
//

import SwiftUI

struct swiftuiscrolltemplate: View {
    var body: some View {
		ScrollView {
			VStack {
				LinearGradient(gradient: Gradient(colors: [Color(.systemGray5), Color(.systemBackground)]), startPoint: .top, endPoint: .bottom)
					.frame(height: 1000)
					.clipped()
					.mask { RoundedRectangle(cornerRadius: 10, style: .continuous) }
					.padding()
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
					Text("IDS")
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
							Image(systemName: "square.and.pencil.circle")
								.imageScale(.large)
								.symbolRenderingMode(.monochrome)
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
    swiftuiscrolltemplate()
}