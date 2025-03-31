//
//  LoginScreen.swift
//  MyProject
//
//  Designed in DetailsPro
//  Copyright © (My Organization). All rights reserved.
//

import SwiftUI

struct LoginScreen: View {
    var body: some View {
		VStack {
			// Top Image
			Image("myImage")
				.renderingMode(.original)
				.resizable()
				.aspectRatio(contentMode: .fill)
				.frame(width: 356, height: 480)
				.clipped()
				.overlay(alignment: .topLeading) {
					// Hero
					VStack(alignment: .leading, spacing: 11) {
						// App Icon
						RoundedRectangle(cornerRadius: 17, style: .continuous)
							.fill(.yellow)
							.frame(width: 72, height: 72)
							.clipped()
							.shadow(color: Color(.sRGBLinear, red: 0/255, green: 0/255, blue: 0/255).opacity(0.12), radius: 8, x: 0, y: 4)
							.overlay {
								Image(systemName: "compass.drawing")
									.imageScale(.large)
									.symbolRenderingMode(.monochrome)
									.font(.system(size: 31, weight: .regular, design: .default))
							}
						VStack(alignment: .leading, spacing: 1) {
							Text("IDS Log")
								.font(.system(.largeTitle, weight: .medium))
							Text("log distance identifier")
								.font(.system(.headline, weight: .medium))
								.frame(width: 190, alignment: .leading)
								.clipped()
								.multilineTextAlignment(.leading)
						}
					}
					.padding()
					.padding(.top, 42)
				}
				.overlay(alignment: .bottom) {
					// Planes Visual
					HStack {
						Spacer()
						ForEach(0..<5) { _ in // Replace with your data model here
							Image(systemName: "memories")
								.symbolRenderingMode(.monochrome)
								.foregroundStyle(Color(.quaternaryLabel))
							Spacer()
						}
					}
					.frame(maxWidth: .infinity)
					.clipped()
					.padding()
					.background {
						Rectangle()
							.fill(.clear)
							.background(Material.thin)
							.mask {
								RoundedRectangle(cornerRadius: 12, style: .continuous)
							}
					}
					.padding()
				}
				.mask {
					RoundedRectangle(cornerRadius: 24, style: .continuous)
				}
				.padding()
				.padding(.top, 40)
				.shadow(color: Color(.sRGBLinear, red: 0/255, green: 0/255, blue: 0/255).opacity(0.15), radius: 18, x: 0, y: 14)
			VStack(spacing: 10) {
				ForEach(0..<5) { _ in // Replace with your data model here
					// Email
					HStack(alignment: .firstTextBaseline) {
						Image(systemName: "person.badge.key")
							.imageScale(.medium)
							.symbolRenderingMode(.monochrome)
						Text("Login")
					}
					.font(.system(.body, weight: .medium))
					.padding(.vertical, 16)
					.frame(maxWidth: .infinity)
					.clipped()
					.foregroundStyle(.orange)
					.background {
						RoundedRectangle(cornerRadius: 10, style: .continuous)
							.stroke(.clear.opacity(0.25), lineWidth: 0)
							.background(RoundedRectangle(cornerRadius: 10, style: .continuous).fill(.yellow.opacity(0.15)))
					}
				}
				Text("Create new database")
					.padding(.top)
					.foregroundStyle(Color(.tertiaryLabel))
					.font(.subheadline)
			}
			.padding(.horizontal)
			Spacer()
		}
    }
}

#Preview {
    LoginScreen()
}